#[doc = "Register `NECNT` reader"]
pub struct R(crate::R<NECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NECNT_SPEC>> for R {
    fn from(reader: crate::R<NECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NECNT` writer"]
pub struct W(crate::W<NECNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<NECNT_SPEC>> for W {
    fn from(writer: crate::W<NECNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REC` reader - Receive Error Counter"]
pub struct REC_R(crate::FieldReader<u8, u8>);
impl REC_R {
    pub(crate) fn new(bits: u8) -> Self {
        REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REC` writer - Receive Error Counter"]
pub struct REC_W<'a> {
    w: &'a mut W,
}
impl<'a> REC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub struct TEC_R(crate::FieldReader<u8, u8>);
impl TEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEC` writer - Transmit Error Counter"]
pub struct TEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `EWRNLVL` reader - Error Warning Level"]
pub struct EWRNLVL_R(crate::FieldReader<u8, u8>);
impl EWRNLVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EWRNLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWRNLVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWRNLVL` writer - Error Warning Level"]
pub struct EWRNLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> EWRNLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Last Error Transfer Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LETD_A {
    #[doc = "0: The last error occurred while the CAN node x was receiver (REC has been incremented)."]
    VALUE1 = 0,
    #[doc = "1: The last error occurred while the CAN node x was transmitter (TEC has been incremented)."]
    VALUE2 = 1,
}
impl From<LETD_A> for bool {
    #[inline(always)]
    fn from(variant: LETD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LETD` reader - Last Error Transfer Direction"]
pub struct LETD_R(crate::FieldReader<bool, LETD_A>);
impl LETD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LETD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LETD_A {
        match self.bits {
            false => LETD_A::VALUE1,
            true => LETD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LETD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LETD_A::VALUE2
    }
}
impl core::ops::Deref for LETD_R {
    type Target = crate::FieldReader<bool, LETD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Last Error Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEINC_A {
    #[doc = "0: The last error led to an error counter increment of 1."]
    VALUE1 = 0,
    #[doc = "1: The last error led to an error counter increment of 8."]
    VALUE2 = 1,
}
impl From<LEINC_A> for bool {
    #[inline(always)]
    fn from(variant: LEINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEINC` reader - Last Error Increment"]
pub struct LEINC_R(crate::FieldReader<bool, LEINC_A>);
impl LEINC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEINC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEINC_A {
        match self.bits {
            false => LEINC_A::VALUE1,
            true => LEINC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LEINC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LEINC_A::VALUE2
    }
}
impl core::ops::Deref for LEINC_R {
    type Target = crate::FieldReader<bool, LEINC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Error Warning Level"]
    #[inline(always)]
    pub fn ewrnlvl(&self) -> EWRNLVL_R {
        EWRNLVL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Last Error Transfer Direction"]
    #[inline(always)]
    pub fn letd(&self) -> LETD_R {
        LETD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Last Error Increment"]
    #[inline(always)]
    pub fn leinc(&self) -> LEINC_R {
        LEINC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&mut self) -> REC_W {
        REC_W { w: self }
    }
    #[doc = "Bits 8:15 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&mut self) -> TEC_W {
        TEC_W { w: self }
    }
    #[doc = "Bits 16:23 - Error Warning Level"]
    #[inline(always)]
    pub fn ewrnlvl(&mut self) -> EWRNLVL_W {
        EWRNLVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [necnt](index.html) module"]
pub struct NECNT_SPEC;
impl crate::RegisterSpec for NECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [necnt::R](R) reader structure"]
impl crate::Readable for NECNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [necnt::W](W) writer structure"]
impl crate::Writable for NECNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NECNT to value 0x0060_0000"]
impl crate::Resettable for NECNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0060_0000
    }
}
