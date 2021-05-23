#[doc = "Register `DIEPDMA0` reader"]
pub struct R(crate::R<DIEPDMA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPDMA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DIEPDMA0_SPEC>> for R {
    fn from(reader: crate::R<DIEPDMA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPDMA0` writer"]
pub struct W(crate::W<DIEPDMA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPDMA0_SPEC>;
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
impl core::convert::From<crate::W<DIEPDMA0_SPEC>> for W {
    fn from(writer: crate::W<DIEPDMA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAAddr` reader - DMA Address"]
pub struct DMAADDR_R(crate::FieldReader<u32, u32>);
impl DMAADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMAADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAAddr` writer - DMA Address"]
pub struct DMAADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W {
        DMAADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdma0](index.html) module"]
pub struct DIEPDMA0_SPEC;
impl crate::RegisterSpec for DIEPDMA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepdma0::R](R) reader structure"]
impl crate::Readable for DIEPDMA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepdma0::W](W) writer structure"]
impl crate::Writable for DIEPDMA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPDMA0 to value 0"]
impl crate::Resettable for DIEPDMA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
