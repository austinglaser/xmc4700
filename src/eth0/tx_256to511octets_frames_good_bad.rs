#[doc = "Register `TX_256TO511OCTETS_FRAMES_GOOD_BAD` reader"]
pub struct R(crate::R<TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>> for R {
    fn from(reader: crate::R<TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX256_511OCTGB` reader - This field indicates the number of transmitted good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
pub struct TX256_511OCTGB_R(crate::FieldReader<u32, u32>);
impl TX256_511OCTGB_R {
    pub(crate) fn new(bits: u32) -> Self {
        TX256_511OCTGB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX256_511OCTGB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx256_511octgb(&self) -> TX256_511OCTGB_R {
        TX256_511OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_256to511octets_frames_good_bad](index.html) module"]
pub struct TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_256to511octets_frames_good_bad::R](R) reader structure"]
impl crate::Readable for TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_256TO511OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
