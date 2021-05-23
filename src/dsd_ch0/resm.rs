#[doc = "Register `RESM` reader"]
pub struct R(crate::R<RESM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RESM_SPEC>> for R {
    fn from(reader: crate::R<RESM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
pub struct RESULT_R(crate::FieldReader<u16, u16>);
impl RESULT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Result Register, Main Filter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resm](index.html) module"]
pub struct RESM_SPEC;
impl crate::RegisterSpec for RESM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resm::R](R) reader structure"]
impl crate::Readable for RESM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESM to value 0"]
impl crate::Resettable for RESM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
