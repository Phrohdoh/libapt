enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum Action
    {
        End = 0x00,
        NextFrame = 0x04,
        PrevFrame = 0x05,
        Play = 0x06,
        Stop = 0x07,
        ToggleQuality = 0x08,
        StopSounds = 0x09,
        Add = 0x0A,
        Subtract = 0x0B,
        Multiply = 0x0C,
        Divide = 0x0D,
        Equals = 0x0E,
        LessThan = 0x0F,
        And = 0x10,
        Or = 0x11,
        Not = 0x12,
        StringEquals = 0x13,
        StringLength = 0x14,
        SubString = 0x15,
        Pop = 0x17,
        ToInt = 0x18,
        GetVariable = 0x1C,
        SetVariable = 0x1D,
        SetTarget2 = 0x20,
        StringConcat = 0x21,
        GetProperty = 0x22,
        SetProperty = 0x23,
        Clone_Sprite = 0x24,
        Remove_Sprite = 0x25,
        Trace = 0x26,
        StartDragMovie = 0x27,
        StopDragMovie = 0x28,
        StringCompare = 0x29,
        Throw = 0x2A,
        CastOp = 0x2B,
        ImplementsOp = 0x2C,
        Random = 0x30,
        MBLength = 0x31, //MB is multibyte strings
        CharToAscii = 0x32,
        AsciiToChar = 0x33,
        GetTimer = 0x34,
        MBSubString = 0x35, //MB is multibyte strings
        MBOrd = 0x36, //MB is multibyte strings
        MBChar = 0x37, //MB is multibyte strings
        Delete = 0x3A,
        Delete2 = 0x3B,
        DefineLocal = 0x3C,
        CallFunction = 0x3D,
        Return = 0x3E,
        Modulo = 0x3F,
        New = 0x40,
        Var = 0x41,
        InitArray = 0x42,
        InitObject = 0x43,
        TypeOf = 0x44,
        TargetPath = 0x45,
        Enumerate = 0x46,
        Add2 = 0x47, //does also handle strings
        LessThan2 = 0x48,
        Equals2 = 0x49,
        ToNumber = 0x4A,
        ToString = 0x4B,
        PushDuplicate = 0x4C,
        StackSwap = 0x4D,
        GetMember = 0x4E,
        SetMember = 0x4F,
        Increment = 0x50,
        Decrement = 0x51,
        CallMethod = 0x52,
        NewMethod = 0x53,
        InstanceOf = 0x54,
        Enumerate2 = 0x55,
        EA_PushThis = 0x56,
        EA_PushGlobal = 0x58,
        EA_PushZero = 0x59,
        EA_PushOne = 0x5A,
        EA_CallFuncNPop = 0x5B,
        EA_CallFunc = 0x5C,
        EA_CallMethodPop = 0x5D,
        EA_CallMethod = 0x5E,
        BitwiseAnd = 0x60,
        BitwiseOr = 0x61,
        BitwiseXor = 0x62,
        ShiftLeft = 0x63,
        ShiftRight = 0x64,
        ShiftRight2 = 0x65,
        StrictEq = 0x66,
        Greater = 0x67,
        StringGreater = 0x68,
        Extends = 0x69,
        EA_PushThisVar = 0x70,
        EA_PushGlobalVar = 0x71,
        EA_ZeroVar = 0x72,
        EA_PushTrue = 0x73,
        EA_PushFalse = 0x74,
        EA_PushNULL = 0x75,
        EA_PushUndefined = 0x76,
        Trace_Start = 0x77,
        GotoFrame = 0x81,
        GetUrl = 0x83,
        SetRegister = 0x87,
        ConstantPool = 0x88,
        WaitForFrame = 0x8A,
        SetTarget = 0x8B,
        GotoLabel = 0x8C,
        WaitForFrameEXPR = 0x8D,
        DefineFunction2 = 0x8E,
        Try = 0x8F,
        With = 0x94,
        PushData = 0x96,
        BranchAlways = 0x99,
        GetUrl2 = 0x9A,
        DefineFunction = 0x9B,
        BranchIfTrue = 0x9D,
        CallFrame = 0x9E,
        GotoFrame2 = 0x9F,     
        EA_PushString = 0xA1,
        EA_PushConstantByte = 0xA2,
        EA_PushConstantWord = 0xA3,
        EA_GetStringVar = 0xA4,
        EA_GetStringMember = 0xA5,
        EA_SetStringVar = 0xA6,
        EA_SetStringMember = 0xA7,
        EA_PushValueOfVar = 0xAE,
        EA_GetNamedMember = 0xAF,
        EA_CallNamedFuncPop = 0xB0,
        EA_CallNamedFunc = 0xB1,
        EA_CallNamedMethodPop = 0xB2,
        EA_CallNamedMethod = 0xB3,
        EA_PushFloat = 0xB4,
        EA_PushByte = 0xB5,
        EA_PushShort = 0xB6,
        EA_PushLong = 0xB7,
        EA_BranchIfFalse = 0xB8,
        EA_PushRegister = 0xB9  
    }
}

pub fn requires_alignment(action: &Action) -> bool {
    use self::Action::*;
    match *action {
        GotoFrame |
        DefineFunction |
        DefineFunction2 |
        ConstantPool |
        BranchIfTrue |
        PushData |
        GetUrl |
        GotoLabel |
        SetRegister |
        EA_PushString |
        EA_GetStringVar |
        EA_GetStringMember => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    extern crate num;
    use self::num::FromPrimitive;

    #[test]
    fn ea_push_one_eq_0x5a() {
        use super::Action;
        assert_eq!(Action::from_u8(0x5a), Some(Action::EA_PushOne));
        assert_eq!(0x5a, Action::EA_PushOne as usize);
    }

    #[test]
    fn requires_alignment() {
        use super::Action::*;
        for action in &[GotoFrame,
                        DefineFunction,
                        DefineFunction2,
                        ConstantPool,
                        BranchIfTrue,
                        PushData,
                        GetUrl,
                        GotoLabel,
                        SetRegister,
                        EA_PushString,
                        EA_GetStringVar,
                        EA_GetStringMember] {
            assert_eq!(super::requires_alignment(action), true);
        }
    }
}
