#[doc = "Register `RBUF0` reader"]
pub struct R(crate::R<RBUF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBUF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RBUF0_SPEC>> for R {
    fn from(reader: crate::R<RBUF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DSR0` reader - Data of Shift Registers 0\\[3:0\\]"]
pub struct DSR0_R(crate::FieldReader<u16, u16>);
impl DSR0_R {
    pub(crate) fn new(bits: u16) -> Self {
        DSR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Data of Shift Registers 0\\[3:0\\]"]
    #[inline(always)]
    pub fn dsr0(&self) -> DSR0_R {
        DSR0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver Buffer Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbuf0](index.html) module"]
pub struct RBUF0_SPEC;
impl crate::RegisterSpec for RBUF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbuf0::R](R) reader structure"]
impl crate::Readable for RBUF0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RBUF0 to value 0"]
impl crate::Resettable for RBUF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
