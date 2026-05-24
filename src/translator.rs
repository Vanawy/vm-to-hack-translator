use crate::command::{ArithmeticCommand, Command, StackOperation};
use crate::segment::Segment;
use indoc::{formatdoc, indoc};

pub struct Translator {
    pub filename: String,
    cmp_counter: u16,
}

const TEMP_BASE_ADDR: u16 = 5;

impl Translator {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            cmp_counter: 0,
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
                vec![pop_data_from_stack(), "D=-D".into(), push_data_to_stack()]
            }
            ArithmeticCommand::Not => {
                vec![pop_data_from_stack(), "D=!D".into(), push_data_to_stack()]
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

    fn advance_counter(&mut self) -> String {
        let res = format!("{}", self.cmp_counter);
        self.cmp_counter += 1;
        res
    }

    fn compare(&mut self, cmd: ArithmeticCommand) -> Vec<String> {
        vec![
            pop_data_from_stack(), // pop arg 1 to D
            formatdoc! {
                "@SP
                AM=M-1
                D=M-D
                @CMP_TRUE_{cmp_counter}
                D;{jump}
                D=0
                @CMP_END_{cmp_counter}
                0;JMP
                (CMP_TRUE_{cmp_counter})
                D=-1
                (CMP_END_{cmp_counter})",
                cmp_counter = self.advance_counter(),
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
