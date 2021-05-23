#[doc = "Register `RX_FRAMES_COUNT_GOOD_BAD` reader"]
pub struct R(crate::R<RX_FRAMES_COUNT_GOOD_BAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FRAMES_COUNT_GOOD_BAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RX_FRAMES_COUNT_GOOD_BAD_SPEC>> for R {
    fn from(reader: crate::R<RX_FRAMES_COUNT_GOOD_BAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFRMGB` reader - This field indicates the number of received good and bad frames."]
pub struct RXFRMGB_R(crate::FieldReader<u32, u32>);
impl RXFRMGB_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXFRMGB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFRMGB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames."]
    #[inline(always)]
    pub fn rxfrmgb(&self) -> RXFRMGB_R {
        RXFRMGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive Frame Count for Good and Bad Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_frames_count_good_bad](index.html) module"]
pub struct RX_FRAMES_COUNT_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_FRAMES_COUNT_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_frames_count_good_bad::R](R) reader structure"]
impl crate::Readable for RX_FRAMES_COUNT_GOOD_BAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FRAMES_COUNT_GOOD_BAD to value 0"]
impl crate::Resettable for RX_FRAMES_COUNT_GOOD_BAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
