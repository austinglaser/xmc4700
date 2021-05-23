#[doc = "Register `DIEPTXF5` reader"]
pub struct R(crate::R<DIEPTXF5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DIEPTXF5_SPEC>> for R {
    fn from(reader: crate::R<DIEPTXF5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTXF5` writer"]
pub struct W(crate::W<DIEPTXF5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF5_SPEC>;
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
impl core::convert::From<crate::W<DIEPTXF5_SPEC>> for W {
    fn from(writer: crate::W<DIEPTXF5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPnTxFStAddr` reader - IN Endpoint FIFOn Transmit RAM Start Address"]
pub struct INEPNTXFSTADDR_R(crate::FieldReader<u16, u16>);
impl INEPNTXFSTADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        INEPNTXFSTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPNTXFSTADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPnTxFStAddr` writer - IN Endpoint FIFOn Transmit RAM Start Address"]
pub struct INEPNTXFSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNTXFSTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `INEPnTxFDep` reader - IN Endpoint TxFIFO Depth"]
pub struct INEPNTXFDEP_R(crate::FieldReader<u16, u16>);
impl INEPNTXFDEP_R {
    pub(crate) fn new(bits: u16) -> Self {
        INEPNTXFDEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPNTXFDEP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPnTxFDep` writer - IN Endpoint TxFIFO Depth"]
pub struct INEPNTXFDEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNTXFDEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN Endpoint FIFOn Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepn_tx_fst_addr(&self) -> INEPNTXFSTADDR_R {
        INEPNTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepn_tx_fdep(&self) -> INEPNTXFDEP_R {
        INEPNTXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN Endpoint FIFOn Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepn_tx_fst_addr(&mut self) -> INEPNTXFSTADDR_W {
        INEPNTXFSTADDR_W { w: self }
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepn_tx_fdep(&mut self) -> INEPNTXFDEP_W {
        INEPNTXFDEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf5](index.html) module"]
pub struct DIEPTXF5_SPEC;
impl crate::RegisterSpec for DIEPTXF5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptxf5::R](R) reader structure"]
impl crate::Readable for DIEPTXF5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptxf5::W](W) writer structure"]
impl crate::Writable for DIEPTXF5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPTXF5 to value 0x0100_052a"]
impl crate::Resettable for DIEPTXF5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_052a
    }
}
