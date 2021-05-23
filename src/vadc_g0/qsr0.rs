#[doc = "Register `QSR0` reader"]
pub struct R(crate::R<QSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<QSR0_SPEC>> for R {
    fn from(reader: crate::R<QSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Filling Level for Queue 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILL_A {
    #[doc = "0: There is 1 ( if EMPTY = 0) or no (if EMPTY = 1) valid entry in the queue"]
    VALUE1 = 0,
    #[doc = "1: There are 2 valid entries in the queue"]
    VALUE2 = 1,
    #[doc = "2: There are 3 valid entries in the queue"]
    VALUE3 = 2,
    #[doc = "7: There are 8 valid entries in the queue"]
    VALUE4 = 7,
}
impl From<FILL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FILL` reader - Filling Level for Queue 2"]
pub struct FILL_R(crate::FieldReader<u8, FILL_A>);
impl FILL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FILL_A> {
        match self.bits {
            0 => Some(FILL_A::VALUE1),
            1 => Some(FILL_A::VALUE2),
            2 => Some(FILL_A::VALUE3),
            7 => Some(FILL_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FILL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FILL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == FILL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == FILL_A::VALUE4
    }
}
impl core::ops::Deref for FILL_R {
    type Target = crate::FieldReader<u8, FILL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Queue Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPTY_A {
    #[doc = "0: There are valid entries in the queue (see FILL)"]
    VALUE1 = 0,
    #[doc = "1: No valid entries (queue is empty)"]
    VALUE2 = 1,
}
impl From<EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMPTY` reader - Queue Empty"]
pub struct EMPTY_R(crate::FieldReader<bool, EMPTY_A>);
impl EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMPTY_A {
        match self.bits {
            false => EMPTY_A::VALUE1,
            true => EMPTY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EMPTY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EMPTY_A::VALUE2
    }
}
impl core::ops::Deref for EMPTY_R {
    type Target = crate::FieldReader<bool, EMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Request Gate Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQGT_A {
    #[doc = "0: The gate input is low"]
    VALUE1 = 0,
    #[doc = "1: The gate input is high"]
    VALUE2 = 1,
}
impl From<REQGT_A> for bool {
    #[inline(always)]
    fn from(variant: REQGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQGT` reader - Request Gate Level"]
pub struct REQGT_R(crate::FieldReader<bool, REQGT_A>);
impl REQGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQGT_A {
        match self.bits {
            false => REQGT_A::VALUE1,
            true => REQGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REQGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REQGT_A::VALUE2
    }
}
impl core::ops::Deref for REQGT_R {
    type Target = crate::FieldReader<bool, REQGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Event Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV_A {
    #[doc = "0: No trigger event"]
    VALUE1 = 0,
    #[doc = "1: A trigger event has been detected"]
    VALUE2 = 1,
}
impl From<EV_A> for bool {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV` reader - Event Detected"]
pub struct EV_R(crate::FieldReader<bool, EV_A>);
impl EV_R {
    pub(crate) fn new(bits: bool) -> Self {
        EV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV_A {
        match self.bits {
            false => EV_A::VALUE1,
            true => EV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EV_A::VALUE2
    }
}
impl core::ops::Deref for EV_R {
    type Target = crate::FieldReader<bool, EV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Filling Level for Queue 2"]
    #[inline(always)]
    pub fn fill(&self) -> FILL_R {
        FILL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Queue Empty"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Request Gate Level"]
    #[inline(always)]
    pub fn reqgt(&self) -> REQGT_R {
        REQGT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Event Detected"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Queue 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qsr0](index.html) module"]
pub struct QSR0_SPEC;
impl crate::RegisterSpec for QSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qsr0::R](R) reader structure"]
impl crate::Readable for QSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QSR0 to value 0x20"]
impl crate::Resettable for QSR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
