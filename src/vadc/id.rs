#[doc = "Register `ID` reader"]
pub struct R(crate::R<ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ID_SPEC>> for R {
    fn from(reader: crate::R<ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MOD_REV` reader - Module Revision"]
pub struct MOD_REV_R(crate::FieldReader<u8, u8>);
impl MOD_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MOD_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD_TYPE` reader - Module Type"]
pub struct MOD_TYPE_R(crate::FieldReader<u8, u8>);
impl MOD_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MOD_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD_NUMBER` reader - Module Number"]
pub struct MOD_NUMBER_R(crate::FieldReader<u16, u16>);
impl MOD_NUMBER_R {
    pub(crate) fn new(bits: u16) -> Self {
        MOD_NUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_NUMBER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn mod_rev(&self) -> MOD_REV_R {
        MOD_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn mod_type(&self) -> MOD_TYPE_R {
        MOD_TYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn mod_number(&self) -> MOD_NUMBER_R {
        MOD_NUMBER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Module Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](index.html) module"]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id::R](R) reader structure"]
impl crate::Readable for ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID to value 0x00c5_c000"]
impl crate::Resettable for ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00c5_c000
    }
}
