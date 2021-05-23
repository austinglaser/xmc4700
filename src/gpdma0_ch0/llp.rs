#[doc = "Register `LLP` reader"]
pub struct R(crate::R<LLP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LLP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LLP_SPEC>> for R {
    fn from(reader: crate::R<LLP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LLP` writer"]
pub struct W(crate::W<LLP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LLP_SPEC>;
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
impl core::convert::From<crate::W<LLP_SPEC>> for W {
    fn from(writer: crate::W<LLP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOC` reader - Starting Address In Memory"]
pub struct LOC_R(crate::FieldReader<u32, u32>);
impl LOC_R {
    pub(crate) fn new(bits: u32) -> Self {
        LOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOC` writer - Starting Address In Memory"]
pub struct LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> LOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Starting Address In Memory"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Starting Address In Memory"]
    #[inline(always)]
    pub fn loc(&mut self) -> LOC_W {
        LOC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Linked List Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [llp](index.html) module"]
pub struct LLP_SPEC;
impl crate::RegisterSpec for LLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [llp::R](R) reader structure"]
impl crate::Readable for LLP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [llp::W](W) writer structure"]
impl crate::Writable for LLP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LLP to value 0"]
impl crate::Resettable for LLP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
