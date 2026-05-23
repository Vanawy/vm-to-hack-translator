use crate::command::{ArithmeticCommand, Command, StackOperation};
use crate::segment::Segment;
use indoc::indoc;

pub struct Translator {
    pub filename: String,
}

const TEMP_BASE_ADDR: u16 = 5;

impl Translator {
    pub fn code(&self, command: Command) -> Vec<String> {
        println!("-> {:?}", command);

        let mut res = vec![format!("//{:?}", command)];
        match command {
            Command::Stack {
                operation,
                segment,
                offset,
            } => self.translate_stack(operation, segment, offset),
            Command::Arithmetic(arithmetic) => self.translate_arithmetic(arithmetic),
        }
        .iter()
        .for_each(|s| {
            println!("{}", s);
            res.push(s.to_string());
        });
        res
    }

    fn get_pointer_address(&self, segment: Segment, offset: u16) -> String {
        match segment {
            // Base addresses
            Segment::Argument => "@ARG".into(),
            Segment::Local => "@LCL".into(),
            Segment::This => "@THIS".into(),
            Segment::That => "@THAT".into(),

            // offset included
            Segment::Static => format!("@{}.{}", self.filename, offset),
            Segment::Constant => format!("@{}", offset),
            Segment::Pointer => match offset {
                0 => "@THIS".into(),
                1 => "@THAT".into(),
                _ => unreachable!(),
            },
            Segment::Temp => format!("@{}", TEMP_BASE_ADDR + offset),
        }
    }
    fn translate_stack(&self, op: StackOperation, segment: Segment, offset: u16) -> Vec<String> {
        match segment {
            Segment::Argument
            | Segment::Local
            | Segment::This
            | Segment::That
            | Segment::Pointer => match op {
                StackOperation::Push => self.segment_push(segment, offset),
                StackOperation::Pop => self.segment_pop(segment, offset),
            },
            Segment::Constant => match op {
                StackOperation::Push => self.constant_push(offset),
                StackOperation::Pop => unreachable!(),
            },
            Segment::Temp => match op {
                StackOperation::Push => self.temp_push(offset),
                StackOperation::Pop => self.temp_pop(offset),
            },
            Segment::Static => match op {
                StackOperation::Push => self.static_push(offset),
                StackOperation::Pop => self.static_pop(offset),
            },
        }
    }

    fn segment_push(&self, segment: Segment, offset: u16) -> Vec<String> {
        // addr = *segment + offset
        let mut res = vec![
            self.get_pointer_address(segment, offset), // @segment
            "D=M".into(),                              // D = *segment
        ];

        if offset > 0 {
            res.push(format!("@{}", offset)); // @offset
            res.push("A=D+A".into()); // addr = *segment + offset
        } else {
            res.push("A=D".into());
        }
        res.push("D=M".into()); // D = *addr

        // *SP = *addr
        res.push(write_data_to_stack());
        // SP++
        res.push(increment_stack_pointer());
        res
    }

    fn translate_arithmetic(&self, command: ArithmeticCommand) -> Vec<String> {
        vec![]
    }

    fn constant_push(&self, offset: u16) -> Vec<String> {
        vec![
            self.get_pointer_address(Segment::Constant, offset),
            "D=A".into(), // D = offset
            write_data_to_stack(),
            increment_stack_pointer(),
        ]
    }

    fn segment_pop(&self, segment: Segment, offset: u16) -> Vec<String> {
        // addr = *segment + offset
        let mut res = vec![
            self.get_pointer_address(segment, offset), // @segment
            "D=M".into(),                              // D = *segment
        ];

        if offset > 0 {
            res.push(format!("@{}", offset)); // @offset
            res.push("D=D+A".into()); // addr = *segment + offset
        }
        res.push("@R13".into());
        res.push("M=D".into()); // addr = *segment + offset

        res.push(decrement_stack_pointer()); // SP--
        res.push("A=M\nD=M".into()); // D = *SP
        res.push("@R13\nA=M\nM=D".into()); // *addr = D
        res
    }

    fn temp_push(&self, offset: u16) -> Vec<String> {
        // *SP = *[base_temp+offset]
        // SP++
        vec![
            self.get_pointer_address(Segment::Temp, offset),
            "D=M".into(),
            write_data_to_stack(),
            increment_stack_pointer(),
        ]
    }

    fn temp_pop(&self, offset: u16) -> Vec<String> {
        // SP--
        // [base_temp+offset] = *SP
        vec![
            decrement_stack_pointer(),                       // SP --
            "D=M".into(),                                    // D = *SP
            self.get_pointer_address(Segment::Temp, offset), // @addr
            "M=D".into(),                                    // addr = D
        ]
    }

    fn static_push(&self, offset: u16) -> Vec<String> {
        // *SP = Foo.i
        // SP ++
        vec![
            self.get_pointer_address(Segment::Static, offset),
            "D=M".into(),
            write_data_to_stack(),
            increment_stack_pointer(),
        ]
    }

    fn static_pop(&self, offset: u16) -> Vec<String> {
        // SP--
        // *Foo.i = *SP
        vec![
            decrement_stack_pointer(),
            "D=M".into(),
            self.get_pointer_address(Segment::Static, offset),
            "M=D".into(),
        ]
    }
}

/// *SP = D
fn write_data_to_stack() -> String {
    indoc! {
        "@SP
        A=M
        M=D"
    }
    .into()
}

/// SP++
fn increment_stack_pointer() -> String {
    indoc! {
        "@SP
        M=M+1"
    }
    .into()
}

/// SP--
fn decrement_stack_pointer() -> String {
    indoc! {
        "@SP
        M=M-1"
    }
    .into()
}
