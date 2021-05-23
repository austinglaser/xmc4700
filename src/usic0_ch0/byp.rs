#[doc = "Register `BYP` reader"]
pub struct R(crate::R<BYP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BYP_SPEC>> for R {
    fn from(reader: crate::R<BYP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BYP` writer"]
pub struct W(crate::W<BYP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYP_SPEC>;
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
impl core::convert::From<crate::W<BYP_SPEC>> for W {
    fn from(writer: crate::W<BYP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDATA` reader - Bypass Data"]
pub struct BDATA_R(crate::FieldReader<u16, u16>);
impl BDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        BDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDATA` writer - Bypass Data"]
pub struct BDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> BDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bypass Data"]
    #[inline(always)]
    pub fn bdata(&self) -> BDATA_R {
        BDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bypass Data"]
    #[inline(always)]
    pub fn bdata(&mut self) -> BDATA_W {
        BDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bypass Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [byp](index.html) module"]
pub struct BYP_SPEC;
impl crate::RegisterSpec for BYP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [byp::R](R) reader structure"]
impl crate::Readable for BYP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [byp::W](W) writer structure"]
impl crate::Writable for BYP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BYP to value 0"]
impl crate::Resettable for BYP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
