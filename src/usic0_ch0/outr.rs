#[doc = "Register `OUTR` reader"]
pub struct R(crate::R<OUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OUTR_SPEC>> for R {
    fn from(reader: crate::R<OUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DSR` reader - Received Data"]
pub struct DSR_R(crate::FieldReader<u16, u16>);
impl DSR_R {
    pub(crate) fn new(bits: u16) -> Self {
        DSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCI` reader - Receiver Control Information"]
pub struct RCI_R(crate::FieldReader<u8, u8>);
impl RCI_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Received Data"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Receiver Control Information"]
    #[inline(always)]
    pub fn rci(&self) -> RCI_R {
        RCI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Receiver Buffer Output Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outr](index.html) module"]
pub struct OUTR_SPEC;
impl crate::RegisterSpec for OUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outr::R](R) reader structure"]
impl crate::Readable for OUTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUTR to value 0"]
impl crate::Resettable for OUTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
