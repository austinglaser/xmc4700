#[doc = "Register `WLB` reader"]
pub struct R(crate::R<WLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WLB_SPEC>> for R {
    fn from(reader: crate::R<WLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WLB` writer"]
pub struct W(crate::W<WLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WLB_SPEC>;
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
impl core::convert::From<crate::W<WLB_SPEC>> for W {
    fn from(writer: crate::W<WLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLB` reader - Window Lower Bound"]
pub struct WLB_R(crate::FieldReader<u32, u32>);
impl WLB_R {
    pub(crate) fn new(bits: u32) -> Self {
        WLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLB` writer - Window Lower Bound"]
pub struct WLB_W<'a> {
    w: &'a mut W,
}
impl<'a> WLB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Window Lower Bound"]
    #[inline(always)]
    pub fn wlb(&self) -> WLB_R {
        WLB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Window Lower Bound"]
    #[inline(always)]
    pub fn wlb(&mut self) -> WLB_W {
        WLB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Window Lower Bound Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wlb](index.html) module"]
pub struct WLB_SPEC;
impl crate::RegisterSpec for WLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wlb::R](R) reader structure"]
impl crate::Readable for WLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wlb::W](W) writer structure"]
impl crate::Writable for WLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WLB to value 0"]
impl crate::Resettable for WLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
