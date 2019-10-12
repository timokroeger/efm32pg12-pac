#[doc = "Reader of register ROUTELOC0"]
pub type R = crate::R<u32, super::ROUTELOC0>;
#[doc = "Writer for register ROUTELOC0"]
pub type W = crate::W<u32, super::ROUTELOC0>;
#[doc = "Register ROUTELOC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0INLOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
    #[doc = "4: Location 4"]
    LOC4,
    #[doc = "5: Location 5"]
    LOC5,
    #[doc = "6: Location 6"]
    LOC6,
    #[doc = "7: Location 7"]
    LOC7,
    #[doc = "8: Location 8"]
    LOC8,
    #[doc = "9: Location 9"]
    LOC9,
    #[doc = "10: Location 10"]
    LOC10,
    #[doc = "11: Location 11"]
    LOC11,
    #[doc = "12: Location 12"]
    LOC12,
    #[doc = "13: Location 13"]
    LOC13,
    #[doc = "14: Location 14"]
    LOC14,
    #[doc = "15: Location 15"]
    LOC15,
    #[doc = "16: Location 16"]
    LOC16,
    #[doc = "17: Location 17"]
    LOC17,
    #[doc = "18: Location 18"]
    LOC18,
    #[doc = "19: Location 19"]
    LOC19,
    #[doc = "20: Location 20"]
    LOC20,
    #[doc = "21: Location 21"]
    LOC21,
    #[doc = "22: Location 22"]
    LOC22,
    #[doc = "23: Location 23"]
    LOC23,
    #[doc = "24: Location 24"]
    LOC24,
    #[doc = "25: Location 25"]
    LOC25,
    #[doc = "26: Location 26"]
    LOC26,
    #[doc = "27: Location 27"]
    LOC27,
    #[doc = "28: Location 28"]
    LOC28,
    #[doc = "29: Location 29"]
    LOC29,
    #[doc = "30: Location 30"]
    LOC30,
    #[doc = "31: Location 31"]
    LOC31,
}
impl From<S0INLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: S0INLOC_A) -> Self {
        match variant {
            S0INLOC_A::LOC0 => 0,
            S0INLOC_A::LOC1 => 1,
            S0INLOC_A::LOC2 => 2,
            S0INLOC_A::LOC3 => 3,
            S0INLOC_A::LOC4 => 4,
            S0INLOC_A::LOC5 => 5,
            S0INLOC_A::LOC6 => 6,
            S0INLOC_A::LOC7 => 7,
            S0INLOC_A::LOC8 => 8,
            S0INLOC_A::LOC9 => 9,
            S0INLOC_A::LOC10 => 10,
            S0INLOC_A::LOC11 => 11,
            S0INLOC_A::LOC12 => 12,
            S0INLOC_A::LOC13 => 13,
            S0INLOC_A::LOC14 => 14,
            S0INLOC_A::LOC15 => 15,
            S0INLOC_A::LOC16 => 16,
            S0INLOC_A::LOC17 => 17,
            S0INLOC_A::LOC18 => 18,
            S0INLOC_A::LOC19 => 19,
            S0INLOC_A::LOC20 => 20,
            S0INLOC_A::LOC21 => 21,
            S0INLOC_A::LOC22 => 22,
            S0INLOC_A::LOC23 => 23,
            S0INLOC_A::LOC24 => 24,
            S0INLOC_A::LOC25 => 25,
            S0INLOC_A::LOC26 => 26,
            S0INLOC_A::LOC27 => 27,
            S0INLOC_A::LOC28 => 28,
            S0INLOC_A::LOC29 => 29,
            S0INLOC_A::LOC30 => 30,
            S0INLOC_A::LOC31 => 31,
        }
    }
}
#[doc = "Reader of field `S0INLOC`"]
pub type S0INLOC_R = crate::R<u8, S0INLOC_A>;
impl S0INLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, S0INLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(S0INLOC_A::LOC0),
            1 => Val(S0INLOC_A::LOC1),
            2 => Val(S0INLOC_A::LOC2),
            3 => Val(S0INLOC_A::LOC3),
            4 => Val(S0INLOC_A::LOC4),
            5 => Val(S0INLOC_A::LOC5),
            6 => Val(S0INLOC_A::LOC6),
            7 => Val(S0INLOC_A::LOC7),
            8 => Val(S0INLOC_A::LOC8),
            9 => Val(S0INLOC_A::LOC9),
            10 => Val(S0INLOC_A::LOC10),
            11 => Val(S0INLOC_A::LOC11),
            12 => Val(S0INLOC_A::LOC12),
            13 => Val(S0INLOC_A::LOC13),
            14 => Val(S0INLOC_A::LOC14),
            15 => Val(S0INLOC_A::LOC15),
            16 => Val(S0INLOC_A::LOC16),
            17 => Val(S0INLOC_A::LOC17),
            18 => Val(S0INLOC_A::LOC18),
            19 => Val(S0INLOC_A::LOC19),
            20 => Val(S0INLOC_A::LOC20),
            21 => Val(S0INLOC_A::LOC21),
            22 => Val(S0INLOC_A::LOC22),
            23 => Val(S0INLOC_A::LOC23),
            24 => Val(S0INLOC_A::LOC24),
            25 => Val(S0INLOC_A::LOC25),
            26 => Val(S0INLOC_A::LOC26),
            27 => Val(S0INLOC_A::LOC27),
            28 => Val(S0INLOC_A::LOC28),
            29 => Val(S0INLOC_A::LOC29),
            30 => Val(S0INLOC_A::LOC30),
            31 => Val(S0INLOC_A::LOC31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == S0INLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == S0INLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == S0INLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == S0INLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == S0INLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == S0INLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == S0INLOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == S0INLOC_A::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == S0INLOC_A::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == S0INLOC_A::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == S0INLOC_A::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == S0INLOC_A::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == S0INLOC_A::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == S0INLOC_A::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == S0INLOC_A::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == S0INLOC_A::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == S0INLOC_A::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == S0INLOC_A::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == S0INLOC_A::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == S0INLOC_A::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == S0INLOC_A::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == S0INLOC_A::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == S0INLOC_A::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == S0INLOC_A::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == S0INLOC_A::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == S0INLOC_A::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == S0INLOC_A::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == S0INLOC_A::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == S0INLOC_A::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == S0INLOC_A::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == S0INLOC_A::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == S0INLOC_A::LOC31
    }
}
#[doc = "Write proxy for field `S0INLOC`"]
pub struct S0INLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> S0INLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0INLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1INLOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
    #[doc = "4: Location 4"]
    LOC4,
    #[doc = "5: Location 5"]
    LOC5,
    #[doc = "6: Location 6"]
    LOC6,
    #[doc = "7: Location 7"]
    LOC7,
    #[doc = "8: Location 8"]
    LOC8,
    #[doc = "9: Location 9"]
    LOC9,
    #[doc = "10: Location 10"]
    LOC10,
    #[doc = "11: Location 11"]
    LOC11,
    #[doc = "12: Location 12"]
    LOC12,
    #[doc = "13: Location 13"]
    LOC13,
    #[doc = "14: Location 14"]
    LOC14,
    #[doc = "15: Location 15"]
    LOC15,
    #[doc = "16: Location 16"]
    LOC16,
    #[doc = "17: Location 17"]
    LOC17,
    #[doc = "18: Location 18"]
    LOC18,
    #[doc = "19: Location 19"]
    LOC19,
    #[doc = "20: Location 20"]
    LOC20,
    #[doc = "21: Location 21"]
    LOC21,
    #[doc = "22: Location 22"]
    LOC22,
    #[doc = "23: Location 23"]
    LOC23,
    #[doc = "24: Location 24"]
    LOC24,
    #[doc = "25: Location 25"]
    LOC25,
    #[doc = "26: Location 26"]
    LOC26,
    #[doc = "27: Location 27"]
    LOC27,
    #[doc = "28: Location 28"]
    LOC28,
    #[doc = "29: Location 29"]
    LOC29,
    #[doc = "30: Location 30"]
    LOC30,
    #[doc = "31: Location 31"]
    LOC31,
}
impl From<S1INLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: S1INLOC_A) -> Self {
        match variant {
            S1INLOC_A::LOC0 => 0,
            S1INLOC_A::LOC1 => 1,
            S1INLOC_A::LOC2 => 2,
            S1INLOC_A::LOC3 => 3,
            S1INLOC_A::LOC4 => 4,
            S1INLOC_A::LOC5 => 5,
            S1INLOC_A::LOC6 => 6,
            S1INLOC_A::LOC7 => 7,
            S1INLOC_A::LOC8 => 8,
            S1INLOC_A::LOC9 => 9,
            S1INLOC_A::LOC10 => 10,
            S1INLOC_A::LOC11 => 11,
            S1INLOC_A::LOC12 => 12,
            S1INLOC_A::LOC13 => 13,
            S1INLOC_A::LOC14 => 14,
            S1INLOC_A::LOC15 => 15,
            S1INLOC_A::LOC16 => 16,
            S1INLOC_A::LOC17 => 17,
            S1INLOC_A::LOC18 => 18,
            S1INLOC_A::LOC19 => 19,
            S1INLOC_A::LOC20 => 20,
            S1INLOC_A::LOC21 => 21,
            S1INLOC_A::LOC22 => 22,
            S1INLOC_A::LOC23 => 23,
            S1INLOC_A::LOC24 => 24,
            S1INLOC_A::LOC25 => 25,
            S1INLOC_A::LOC26 => 26,
            S1INLOC_A::LOC27 => 27,
            S1INLOC_A::LOC28 => 28,
            S1INLOC_A::LOC29 => 29,
            S1INLOC_A::LOC30 => 30,
            S1INLOC_A::LOC31 => 31,
        }
    }
}
#[doc = "Reader of field `S1INLOC`"]
pub type S1INLOC_R = crate::R<u8, S1INLOC_A>;
impl S1INLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, S1INLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(S1INLOC_A::LOC0),
            1 => Val(S1INLOC_A::LOC1),
            2 => Val(S1INLOC_A::LOC2),
            3 => Val(S1INLOC_A::LOC3),
            4 => Val(S1INLOC_A::LOC4),
            5 => Val(S1INLOC_A::LOC5),
            6 => Val(S1INLOC_A::LOC6),
            7 => Val(S1INLOC_A::LOC7),
            8 => Val(S1INLOC_A::LOC8),
            9 => Val(S1INLOC_A::LOC9),
            10 => Val(S1INLOC_A::LOC10),
            11 => Val(S1INLOC_A::LOC11),
            12 => Val(S1INLOC_A::LOC12),
            13 => Val(S1INLOC_A::LOC13),
            14 => Val(S1INLOC_A::LOC14),
            15 => Val(S1INLOC_A::LOC15),
            16 => Val(S1INLOC_A::LOC16),
            17 => Val(S1INLOC_A::LOC17),
            18 => Val(S1INLOC_A::LOC18),
            19 => Val(S1INLOC_A::LOC19),
            20 => Val(S1INLOC_A::LOC20),
            21 => Val(S1INLOC_A::LOC21),
            22 => Val(S1INLOC_A::LOC22),
            23 => Val(S1INLOC_A::LOC23),
            24 => Val(S1INLOC_A::LOC24),
            25 => Val(S1INLOC_A::LOC25),
            26 => Val(S1INLOC_A::LOC26),
            27 => Val(S1INLOC_A::LOC27),
            28 => Val(S1INLOC_A::LOC28),
            29 => Val(S1INLOC_A::LOC29),
            30 => Val(S1INLOC_A::LOC30),
            31 => Val(S1INLOC_A::LOC31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == S1INLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == S1INLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == S1INLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == S1INLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == S1INLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == S1INLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == S1INLOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == S1INLOC_A::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == S1INLOC_A::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == S1INLOC_A::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == S1INLOC_A::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == S1INLOC_A::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == S1INLOC_A::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == S1INLOC_A::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == S1INLOC_A::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == S1INLOC_A::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == S1INLOC_A::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == S1INLOC_A::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == S1INLOC_A::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == S1INLOC_A::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == S1INLOC_A::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == S1INLOC_A::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == S1INLOC_A::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == S1INLOC_A::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == S1INLOC_A::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == S1INLOC_A::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == S1INLOC_A::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == S1INLOC_A::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == S1INLOC_A::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == S1INLOC_A::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == S1INLOC_A::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == S1INLOC_A::LOC31
    }
}
#[doc = "Write proxy for field `S1INLOC`"]
pub struct S1INLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> S1INLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1INLOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn s0inloc(&self) -> S0INLOC_R {
        S0INLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn s1inloc(&self) -> S1INLOC_R {
        S1INLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn s0inloc(&mut self) -> S0INLOC_W {
        S0INLOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn s1inloc(&mut self) -> S1INLOC_W {
        S1INLOC_W { w: self }
    }
}
