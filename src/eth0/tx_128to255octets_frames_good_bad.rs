#[doc = "Register `TX_128TO255OCTETS_FRAMES_GOOD_BAD` reader"]
pub struct R(crate::R<TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>> for R {
    fn from(reader: crate::R<TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX128_255OCTGB` reader - This field indicates the number of transmitted good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
pub struct TX128_255OCTGB_R(crate::FieldReader<u32, u32>);
impl TX128_255OCTGB_R {
    pub(crate) fn new(bits: u32) -> Self {
        TX128_255OCTGB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX128_255OCTGB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub fn tx128_255octgb(&self) -> TX128_255OCTGB_R {
        TX128_255OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_128to255octets_frames_good_bad](index.html) module"]
pub struct TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_128to255octets_frames_good_bad::R](R) reader structure"]
impl crate::Readable for TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_128TO255OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
