use crate::command::{
    ArithmeticCommand, BranchingOperation, Command, FunctionStatement, StackOperation,
};
use crate::segment::Segment;
use indoc::{formatdoc, indoc};

pub struct Translator {
    pub filename: String,
    cmp_counter: u16,
    ret_counter: u16,
}

const TEMP_BASE_ADDR: u16 = 5;
const STACK_FRAME_SIZE: u16 = 5;

impl Translator {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            cmp_counter: 0,
            ret_counter: 0,
        }
    }

    pub fn code(&mut self, command: Command) -> Vec<String> {
        // println!("-> {}", command);

        let mut res = vec![format!("// {}", command)];
        match command {
            Command::Stack {
                operation,
                segment,
                index,
            } => self.translate_stack(operation, segment, index),
            Command::Arithmetic(arithmetic) => self.translate_arithmetic(arithmetic),
            Command::BranchingOperation { operation, label } => {
                self.translate_branching(operation, &label)
            }
            Command::Function {
                statement,
                name,
                n_args,
            } => self.translate_function(statement, &name, n_args),
            Command::Return => self.translate_return(),
        }
        .iter()
        .for_each(|s| {
            // println!("{}", s);
            res.push(s.to_string());
        });
        res
    }

    fn get_pointer_address(&self, segment: Segment, index: u16) -> String {
        match segment {
            // Base addresses
            Segment::Argument => "@ARG".into(),
            Segment::Local => "@LCL".into(),
            Segment::This => "@THIS".into(),
            Segment::That => "@THAT".into(),

            // index included
            Segment::Static => format!("@{}.{}", self.filename, index),
            Segment::Constant => format!("@{}", index),
            Segment::Pointer => match index {
                0 => "@THIS".into(),
                1 => "@THAT".into(),
                _ => unreachable!(),
            },
            Segment::Temp => format!("@{}", TEMP_BASE_ADDR + index),
        }
    }

    fn translate_stack(&self, op: StackOperation, segment: Segment, index: u16) -> Vec<String> {
        match segment {
            Segment::Argument | Segment::Local | Segment::This | Segment::That => match op {
                StackOperation::Push => self.segment_push(segment, index),
                StackOperation::Pop => self.segment_pop(segment, index),
            },
            Segment::Constant => match op {
                StackOperation::Push => self.constant_push(index),
                StackOperation::Pop => unreachable!(),
            },
            Segment::Temp | Segment::Pointer | Segment::Static => match op {
                StackOperation::Push => self.special_segment_push(segment, index),
                StackOperation::Pop => self.special_segment_pop(segment, index),
            },
        }
    }

    fn segment_push(&self, segment: Segment, index: u16) -> Vec<String> {
        // addr = *segment + index
        let mut res = vec![
            self.get_pointer_address(segment, index), // @segment
            "D=M".into(),                             // D = *segment
        ];

        if index > 0 {
            res.push(format!("@{}", index)); // @index
            res.push("A=D+A".into()); // addr = *segment + index
        } else {
            res.push("A=D".into());
        }
        res.push("D=M".into()); // D = *addr
        res.push(push_data_to_stack());
        res
    }

    fn translate_arithmetic(&mut self, command: ArithmeticCommand) -> Vec<String> {
        match command {
            ArithmeticCommand::Negate => {
                vec![
                    indoc! {
                        "@SP
                        A=M-1
                        M=-M
                        "
                    }
                    .into(),
                ]
            }
            ArithmeticCommand::Not => {
                vec![
                    indoc! {
                        "@SP
                        A=M-1
                        M=!M
                        "
                    }
                    .into(),
                ]
            }
            ArithmeticCommand::Equals
            | ArithmeticCommand::GreaterThan
            | ArithmeticCommand::LessThan => self.compare(command),
            ArithmeticCommand::Add => self.binary_op("D=D+M"),
            ArithmeticCommand::Subtract => self.binary_op("D=M-D"),
            ArithmeticCommand::And => self.binary_op("D=D&M"),
            ArithmeticCommand::Or => self.binary_op("D=D|M"),
        }
    }

    fn binary_op(&self, operation: &str) -> Vec<String> {
        vec![
            pop_data_from_stack(),
            indoc! {
                "@SP
                AM=M-1"
            }
            .into(),
            operation.into(),
            push_data_to_stack(),
        ]
    }

    fn constant_push(&self, index: u16) -> Vec<String> {
        vec![
            self.get_pointer_address(Segment::Constant, index),
            "D=A".into(), // D = index
            push_data_to_stack(),
        ]
    }

    fn segment_pop(&self, segment: Segment, index: u16) -> Vec<String> {
        // addr = *segment + index
        let mut res = vec![
            self.get_pointer_address(segment, index), // @segment
            "D=M".into(),                             // D = *segment
        ];

        if index > 0 {
            res.push(format!("@{}", index)); // @index
            res.push("D=D+A".into()); // addr = *segment + index
        }
        res.push("@R13".into());
        res.push("M=D".into()); // addr = *segment + index

        res.push(pop_data_from_stack()); // SP--
        res.push("@R13\nA=M\nM=D".into()); // *addr = D
        res
    }

    fn special_segment_push(&self, segment: Segment, index: u16) -> Vec<String> {
        vec![
            self.get_pointer_address(segment, index),
            "D=M".into(),
            push_data_to_stack(),
        ]
    }

    fn special_segment_pop(&self, segment: Segment, index: u16) -> Vec<String> {
        vec![
            pop_data_from_stack(),                    // SP -- // D = *SP
            self.get_pointer_address(segment, index), // @addr
            "M=D".into(),                             // addr = D
        ]
    }

    fn advance_cmp_counter(&mut self) -> u16 {
        let res = self.cmp_counter.clone();
        self.cmp_counter += 1;
        res
    }

    fn advance_ret_counter(&mut self) -> u16 {
        let res = self.ret_counter.clone();
        self.ret_counter += 1;
        res
    }

    fn compare(&mut self, cmd: ArithmeticCommand) -> Vec<String> {
        vec![
            pop_data_from_stack(), // pop arg 1 to D
            formatdoc! {
                "@SP
                AM=M-1
                D=M-D
                @{filename}$cmpTrue.{cmp_counter}
                D;{jump}
                D=0
                @{filename}$cmpEnd.{cmp_counter}
                0;JMP
                ({filename}$cmpTrue.{cmp_counter})
                D=-1
                ({filename}$cmpEnd.{cmp_counter})",
                filename = self.filename.to_owned(),
                cmp_counter = self.advance_cmp_counter(),
                jump = match cmd {
                    ArithmeticCommand::Equals => "JEQ",
                    ArithmeticCommand::GreaterThan => "JGT",
                    ArithmeticCommand::LessThan => "JLT",
                    _ => unreachable!(),
                }
            },
            push_data_to_stack(),
        ]
    }

    fn translate_branching(&self, operation: BranchingOperation, label: &str) -> Vec<String> {
        let unique_label = format!("{}${}", self.filename, label);
        match operation {
            BranchingOperation::Label => vec![format!("({})", unique_label)],
            BranchingOperation::Goto => vec![formatdoc! {
                "@{label}
                0;JMP",
                label = unique_label
            }],
            BranchingOperation::IfGoto => vec![
                pop_data_from_stack(),
                formatdoc! {
                    "@{label}
                    D;JGT",
                    label = unique_label
                },
            ],
        }
    }

    fn translate_function(
        &mut self,
        statement: FunctionStatement,
        name: &str,
        n_args: u16,
    ) -> Vec<String> {
        match statement {
            FunctionStatement::Return => unreachable!(),
            FunctionStatement::Call => self.translate_function_call(name, n_args),
            FunctionStatement::Declaration => self.translate_function_declaration(name, n_args),
        }
    }

    /**
     * // return
     * Gets the saved return address (which, in this example, happens to be
     * Foo.bar$ret.1), replaces the arguments pushed by the caller with
     * the return value (stack’s top element), reinstates the segment pointers
     * of the caller, and then generates:
     * goto Foo.bar$ret.1 // injected branching back to the calling site.
     */
    fn translate_return(&self) -> Vec<String> {
        let local_addr = self.get_pointer_address(Segment::Local, 0);
        // endFrame = LCL
        let end_frame = "@R14"; // TODO: extract all @Rn registers to constants
        // retAddr = *(endFrame – 5)
        let ret_addr = "@R15";
        [
            vec![formatdoc! {
                "// endFrame = LCL
                {local_addr}
                D=M
                {end_frame}
                M=D
                // retAddr = *(endFrame – 5)
                @{STACK_FRAME_SIZE}
                A=D-A
                D=M
                {ret_addr}
                M=D
                "

            }],
            self.segment_pop(Segment::Argument, 0),
            vec![formatdoc! {
                "// > SP = ARG + 1
                {arg_addr}
                D=M+1
                @SP
                M=D

                // > THAT = *(endFrame - 1)
                {end_frame}
                D=M

                AM=M-1
                D=M
                @THAT
                M=D

                // > THIS = *(endFrame - 2)
                {end_frame}
                AM=M-1
                D=M
                @THIS
                M=D

                // > ARG = *(endFrame - 3)
                {end_frame}
                AM=M-1
                D=M
                @ARG
                M=D

                // > LCL = *(endFrame - 4)
                {end_frame}
                AM=M-1
                D=M
                @LCL
                M=D

                // goto retAddr
                {ret_addr}
                A=M
                0;JMP
                ",
                arg_addr = self.get_pointer_address(Segment::Argument, 0),
            }],
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    /**
     * call Foo.mult 2
     *      name     n_args
     *
     * creates a return address label (Foo.bar$ret.1); Saves the return
     * address and the caller’s segment pointers, and then generates:
     * goto Foo.mult // injected branching to the called function
     * (Foo.bar$ret.1) // injected return address label
     */
    fn translate_function_call(&mut self, name: &str, n_args: u16) -> Vec<String> {
        let ret_counter = self.advance_ret_counter();
        let return_label = format!("{}$return.{}", self.filename, ret_counter);
        [
            self.push_label(&return_label),
            self.segment_push(Segment::Local, 0),
            self.segment_push(Segment::Argument, 0),
            self.segment_push(Segment::This, 0),
            self.segment_push(Segment::That, 0),
            vec![
                // ARG = SP-5-nArgs
                formatdoc! {
                    "@{arg_offset}
                    D=A
                    @SP
                    D=M-D
                    {arg_addr}
                    M=D
                    ",
                    arg_offset = STACK_FRAME_SIZE + n_args,
                    arg_addr = self.get_pointer_address(Segment::Argument, 0),
                },
                formatdoc! {
                    "
                    // LCL = SP
                    @SP
                    D=M
                    {local_addr}
                    M=D
                    ",
                    local_addr = self.get_pointer_address(Segment::Local, 0)
                },
            ],
            self.translate_branching(BranchingOperation::Goto, name.into()),
            self.translate_branching(BranchingOperation::Label, &return_label),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    /**
     * function Foo.mult 2
     * (Foo.mult) // injected function’s entry point label
     * assembly code that initializes the function’s 2 local variables
     */
    fn translate_function_declaration(&self, name: &str, n_args: u16) -> Vec<String> {
        let mut instructions = self.translate_branching(BranchingOperation::Label, name.into());

        let push_zero = formatdoc! {
            "@0
            D=A
            {push_data_to_stack}
            ",
            push_data_to_stack = push_data_to_stack()
        };

        instructions.extend((0..n_args).map(|_| push_zero.to_owned()));
        instructions
    }

    fn push_label(&self, label: &str) -> Vec<String> {
        // addr = @label
        let mut res = vec![
            format!("@{}", label), // @label
        ];
        res.push("D=A".into()); // D =
        res.push(push_data_to_stack());
        res
    }
}

/// *SP = D
/// <p>SP++
fn push_data_to_stack() -> String {
    indoc! {
        "@SP
        A=M
        M=D
        @SP
        M=M+1"
    }
    .into()
}

/// SP--
/// <p>D = *SP
fn pop_data_from_stack() -> String {
    indoc! {
        "@SP
        AM=M-1
        D=M"
    }
    .into()
}
