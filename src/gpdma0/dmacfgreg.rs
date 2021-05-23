#[doc = "Register `DMACFGREG` reader"]
pub struct R(crate::R<DMACFGREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACFGREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMACFGREG_SPEC>> for R {
    fn from(reader: crate::R<DMACFGREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACFGREG` writer"]
pub struct W(crate::W<DMACFGREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACFGREG_SPEC>;
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
impl core::convert::From<crate::W<DMACFGREG_SPEC>> for W {
    fn from(writer: crate::W<DMACFGREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPDMA Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_EN_A {
    #[doc = "0: GPDMA Disabled"]
    VALUE1 = 0,
    #[doc = "1: GPDMA Enabled."]
    VALUE2 = 1,
}
impl From<DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_EN` reader - GPDMA Enable bit."]
pub struct DMA_EN_R(crate::FieldReader<bool, DMA_EN_A>);
impl DMA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_EN_A {
        match self.bits {
            false => DMA_EN_A::VALUE1,
            true => DMA_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DMA_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DMA_EN_A::VALUE2
    }
}
impl core::ops::Deref for DMA_EN_R {
    type Target = crate::FieldReader<bool, DMA_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_EN` writer - GPDMA Enable bit."]
pub struct DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "GPDMA Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMA_EN_A::VALUE1)
    }
    #[doc = "GPDMA Enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMA_EN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPDMA Enable bit."]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA Enable bit."]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DMA_EN_W {
        DMA_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacfgreg](index.html) module"]
pub struct DMACFGREG_SPEC;
impl crate::RegisterSpec for DMACFGREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacfgreg::R](R) reader structure"]
impl crate::Readable for DMACFGREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacfgreg::W](W) writer structure"]
impl crate::Writable for DMACFGREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACFGREG to value 0"]
impl crate::Resettable for DMACFGREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
