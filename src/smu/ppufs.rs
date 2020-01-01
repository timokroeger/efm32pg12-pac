#[doc = "Reader of register PPUFS"]
pub type R = crate::R<u32, super::PPUFS>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERIPHID_A {
    #[doc = "0: Analog Comparator 0"]
    ACMP0 = 0,
    #[doc = "1: Analog Comparator 1"]
    ACMP1 = 1,
    #[doc = "2: Analog to Digital Converter 0"]
    ADC0 = 2,
    #[doc = "5: Clock Management Unit"]
    CMU = 5,
    #[doc = "7: CRYOTIMER"]
    CRYOTIMER = 7,
    #[doc = "8: Advanced Encryption Standard Accelerator 0"]
    CRYPTO0 = 8,
    #[doc = "9: Advanced Encryption Standard Accelerator 1"]
    CRYPTO1 = 9,
    #[doc = "10: Capacitive touch sense module"]
    CSEN = 10,
    #[doc = "11: Digital to Analog Converter 0"]
    VDAC0 = 11,
    #[doc = "12: Peripheral Reflex System"]
    PRS = 12,
    #[doc = "13: Energy Management Unit"]
    EMU = 13,
    #[doc = "14: FPU Exception Handler"]
    FPUEH = 14,
    #[doc = "16: General Purpose CRC"]
    GPCRC = 16,
    #[doc = "17: General purpose Input/Output"]
    GPIO = 17,
    #[doc = "18: I2C 0"]
    I2C0 = 18,
    #[doc = "19: I2C 1"]
    I2C1 = 19,
    #[doc = "20: Current Digital to Analog Converter 0"]
    IDAC0 = 20,
    #[doc = "21: Memory System Controller"]
    MSC = 21,
    #[doc = "22: Linked Direct Memory Access Controller"]
    LDMA = 22,
    #[doc = "23: Low Energy Sensor Interface"]
    LESENSE = 23,
    #[doc = "24: Low Energy Timer 0"]
    LETIMER0 = 24,
    #[doc = "25: Low Energy UART 0"]
    LEUART0 = 25,
    #[doc = "27: Pulse Counter 0"]
    PCNT0 = 27,
    #[doc = "28: Pulse Counter 1"]
    PCNT1 = 28,
    #[doc = "29: Pulse Counter 2"]
    PCNT2 = 29,
    #[doc = "33: Reset Management Unit"]
    RMU = 33,
    #[doc = "34: Real-Time Counter and Calendar"]
    RTCC = 34,
    #[doc = "35: Security Management Unit"]
    SMU = 35,
    #[doc = "37: Timer 0"]
    TIMER0 = 37,
    #[doc = "38: Timer 1"]
    TIMER1 = 38,
    #[doc = "39: True Random Number Generator 0"]
    TRNG0 = 39,
    #[doc = "40: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 40,
    #[doc = "41: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 41,
    #[doc = "42: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2 = 42,
    #[doc = "43: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3 = 43,
    #[doc = "44: Watchdog 0"]
    WDOG0 = 44,
    #[doc = "45: Watchdog 1"]
    WDOG1 = 45,
    #[doc = "46: Wide Timer 0"]
    WTIMER0 = 46,
    #[doc = "47: Wide Timer 1"]
    WTIMER1 = 47,
}
impl From<PERIPHID_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIPHID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PERIPHID`"]
pub type PERIPHID_R = crate::R<u8, PERIPHID_A>;
impl PERIPHID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PERIPHID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PERIPHID_A::ACMP0),
            1 => Val(PERIPHID_A::ACMP1),
            2 => Val(PERIPHID_A::ADC0),
            5 => Val(PERIPHID_A::CMU),
            7 => Val(PERIPHID_A::CRYOTIMER),
            8 => Val(PERIPHID_A::CRYPTO0),
            9 => Val(PERIPHID_A::CRYPTO1),
            10 => Val(PERIPHID_A::CSEN),
            11 => Val(PERIPHID_A::VDAC0),
            12 => Val(PERIPHID_A::PRS),
            13 => Val(PERIPHID_A::EMU),
            14 => Val(PERIPHID_A::FPUEH),
            16 => Val(PERIPHID_A::GPCRC),
            17 => Val(PERIPHID_A::GPIO),
            18 => Val(PERIPHID_A::I2C0),
            19 => Val(PERIPHID_A::I2C1),
            20 => Val(PERIPHID_A::IDAC0),
            21 => Val(PERIPHID_A::MSC),
            22 => Val(PERIPHID_A::LDMA),
            23 => Val(PERIPHID_A::LESENSE),
            24 => Val(PERIPHID_A::LETIMER0),
            25 => Val(PERIPHID_A::LEUART0),
            27 => Val(PERIPHID_A::PCNT0),
            28 => Val(PERIPHID_A::PCNT1),
            29 => Val(PERIPHID_A::PCNT2),
            33 => Val(PERIPHID_A::RMU),
            34 => Val(PERIPHID_A::RTCC),
            35 => Val(PERIPHID_A::SMU),
            37 => Val(PERIPHID_A::TIMER0),
            38 => Val(PERIPHID_A::TIMER1),
            39 => Val(PERIPHID_A::TRNG0),
            40 => Val(PERIPHID_A::USART0),
            41 => Val(PERIPHID_A::USART1),
            42 => Val(PERIPHID_A::USART2),
            43 => Val(PERIPHID_A::USART3),
            44 => Val(PERIPHID_A::WDOG0),
            45 => Val(PERIPHID_A::WDOG1),
            46 => Val(PERIPHID_A::WTIMER0),
            47 => Val(PERIPHID_A::WTIMER1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == PERIPHID_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == PERIPHID_A::ACMP1
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == PERIPHID_A::ADC0
    }
    #[doc = "Checks if the value of the field is `CMU`"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == PERIPHID_A::CMU
    }
    #[doc = "Checks if the value of the field is `CRYOTIMER`"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == PERIPHID_A::CRYOTIMER
    }
    #[doc = "Checks if the value of the field is `CRYPTO0`"]
    #[inline(always)]
    pub fn is_crypto0(&self) -> bool {
        *self == PERIPHID_A::CRYPTO0
    }
    #[doc = "Checks if the value of the field is `CRYPTO1`"]
    #[inline(always)]
    pub fn is_crypto1(&self) -> bool {
        *self == PERIPHID_A::CRYPTO1
    }
    #[doc = "Checks if the value of the field is `CSEN`"]
    #[inline(always)]
    pub fn is_csen(&self) -> bool {
        *self == PERIPHID_A::CSEN
    }
    #[doc = "Checks if the value of the field is `VDAC0`"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == PERIPHID_A::VDAC0
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == PERIPHID_A::PRS
    }
    #[doc = "Checks if the value of the field is `EMU`"]
    #[inline(always)]
    pub fn is_emu(&self) -> bool {
        *self == PERIPHID_A::EMU
    }
    #[doc = "Checks if the value of the field is `FPUEH`"]
    #[inline(always)]
    pub fn is_fpueh(&self) -> bool {
        *self == PERIPHID_A::FPUEH
    }
    #[doc = "Checks if the value of the field is `GPCRC`"]
    #[inline(always)]
    pub fn is_gpcrc(&self) -> bool {
        *self == PERIPHID_A::GPCRC
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == PERIPHID_A::GPIO
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == PERIPHID_A::I2C0
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == PERIPHID_A::I2C1
    }
    #[doc = "Checks if the value of the field is `IDAC0`"]
    #[inline(always)]
    pub fn is_idac0(&self) -> bool {
        *self == PERIPHID_A::IDAC0
    }
    #[doc = "Checks if the value of the field is `MSC`"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == PERIPHID_A::MSC
    }
    #[doc = "Checks if the value of the field is `LDMA`"]
    #[inline(always)]
    pub fn is_ldma(&self) -> bool {
        *self == PERIPHID_A::LDMA
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == PERIPHID_A::LESENSE
    }
    #[doc = "Checks if the value of the field is `LETIMER0`"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == PERIPHID_A::LETIMER0
    }
    #[doc = "Checks if the value of the field is `LEUART0`"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == PERIPHID_A::LEUART0
    }
    #[doc = "Checks if the value of the field is `PCNT0`"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == PERIPHID_A::PCNT0
    }
    #[doc = "Checks if the value of the field is `PCNT1`"]
    #[inline(always)]
    pub fn is_pcnt1(&self) -> bool {
        *self == PERIPHID_A::PCNT1
    }
    #[doc = "Checks if the value of the field is `PCNT2`"]
    #[inline(always)]
    pub fn is_pcnt2(&self) -> bool {
        *self == PERIPHID_A::PCNT2
    }
    #[doc = "Checks if the value of the field is `RMU`"]
    #[inline(always)]
    pub fn is_rmu(&self) -> bool {
        *self == PERIPHID_A::RMU
    }
    #[doc = "Checks if the value of the field is `RTCC`"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == PERIPHID_A::RTCC
    }
    #[doc = "Checks if the value of the field is `SMU`"]
    #[inline(always)]
    pub fn is_smu(&self) -> bool {
        *self == PERIPHID_A::SMU
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == PERIPHID_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == PERIPHID_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `TRNG0`"]
    #[inline(always)]
    pub fn is_trng0(&self) -> bool {
        *self == PERIPHID_A::TRNG0
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == PERIPHID_A::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == PERIPHID_A::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == PERIPHID_A::USART2
    }
    #[doc = "Checks if the value of the field is `USART3`"]
    #[inline(always)]
    pub fn is_usart3(&self) -> bool {
        *self == PERIPHID_A::USART3
    }
    #[doc = "Checks if the value of the field is `WDOG0`"]
    #[inline(always)]
    pub fn is_wdog0(&self) -> bool {
        *self == PERIPHID_A::WDOG0
    }
    #[doc = "Checks if the value of the field is `WDOG1`"]
    #[inline(always)]
    pub fn is_wdog1(&self) -> bool {
        *self == PERIPHID_A::WDOG1
    }
    #[doc = "Checks if the value of the field is `WTIMER0`"]
    #[inline(always)]
    pub fn is_wtimer0(&self) -> bool {
        *self == PERIPHID_A::WTIMER0
    }
    #[doc = "Checks if the value of the field is `WTIMER1`"]
    #[inline(always)]
    pub fn is_wtimer1(&self) -> bool {
        *self == PERIPHID_A::WTIMER1
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn periphid(&self) -> PERIPHID_R {
        PERIPHID_R::new((self.bits & 0x7f) as u8)
    }
}
