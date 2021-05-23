#[doc = "Register `RX_FIFO_OVERFLOW_FRAMES` reader"]
pub struct R(crate::R<RX_FIFO_OVERFLOW_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_OVERFLOW_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RX_FIFO_OVERFLOW_FRAMES_SPEC>> for R {
    fn from(reader: crate::R<RX_FIFO_OVERFLOW_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFOOVFL` reader - This field indicates the number of received frames missed because of FIFO overflow."]
pub struct RXFIFOOVFL_R(crate::FieldReader<u32, u32>);
impl RXFIFOOVFL_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXFIFOOVFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOOVFL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received frames missed because of FIFO overflow."]
    #[inline(always)]
    pub fn rxfifoovfl(&self) -> RXFIFOOVFL_R {
        RXFIFOOVFL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive Frame Count for FIFO Overflow Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_overflow_frames](index.html) module"]
pub struct RX_FIFO_OVERFLOW_FRAMES_SPEC;
impl crate::RegisterSpec for RX_FIFO_OVERFLOW_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_overflow_frames::R](R) reader structure"]
impl crate::Readable for RX_FIFO_OVERFLOW_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FIFO_OVERFLOW_FRAMES to value 0"]
impl crate::Resettable for RX_FIFO_OVERFLOW_FRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
