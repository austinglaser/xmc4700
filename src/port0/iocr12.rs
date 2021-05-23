#[doc = "Register `IOCR12` reader"]
pub struct R(crate::R<IOCR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IOCR12_SPEC>> for R {
    fn from(reader: crate::R<IOCR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCR12` writer"]
pub struct W(crate::W<IOCR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCR12_SPEC>;
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
impl core::convert::From<crate::W<IOCR12_SPEC>> for W {
    fn from(writer: crate::W<IOCR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Control for Port n Pin 12 to 15"]
pub type PC12_A = super::iocr0::PC0_A;
#[doc = "Field `PC12` reader - Port Control for Port n Pin 12 to 15"]
pub type PC12_R = super::iocr0::PC0_R;
#[doc = "Field `PC12` writer - Port Control for Port n Pin 12 to 15"]
pub struct PC12_W<'a> {
    w: &'a mut W,
}
impl<'a> PC12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC12_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC12_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC12_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC12_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC12_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC12_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC12_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC12_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC12_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC12_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC12_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC12_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC12_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC12_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC12_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC12_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC12_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC12_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 12 to 15"]
pub type PC13_A = super::iocr0::PC0_A;
#[doc = "Field `PC13` reader - Port Control for Port n Pin 12 to 15"]
pub type PC13_R = super::iocr0::PC0_R;
#[doc = "Field `PC13` writer - Port Control for Port n Pin 12 to 15"]
pub struct PC13_W<'a> {
    w: &'a mut W,
}
impl<'a> PC13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC13_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC13_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC13_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC13_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC13_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC13_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC13_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC13_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC13_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC13_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC13_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC13_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC13_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC13_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC13_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC13_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC13_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC13_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 12 to 15"]
pub type PC14_A = super::iocr0::PC0_A;
#[doc = "Field `PC14` reader - Port Control for Port n Pin 12 to 15"]
pub type PC14_R = super::iocr0::PC0_R;
#[doc = "Field `PC14` writer - Port Control for Port n Pin 12 to 15"]
pub struct PC14_W<'a> {
    w: &'a mut W,
}
impl<'a> PC14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC14_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC14_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC14_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC14_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC14_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC14_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC14_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC14_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC14_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC14_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC14_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC14_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC14_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC14_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC14_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC14_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC14_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC14_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 12 to 15"]
pub type PC15_A = super::iocr0::PC0_A;
#[doc = "Field `PC15` reader - Port Control for Port n Pin 12 to 15"]
pub type PC15_R = super::iocr0::PC0_R;
#[doc = "Field `PC15` writer - Port Control for Port n Pin 12 to 15"]
pub struct PC15_W<'a> {
    w: &'a mut W,
}
impl<'a> PC15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC15_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC15_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC15_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC15_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC15_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC15_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC15_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC15_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC15_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC15_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC15_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC15_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC15_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC15_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC15_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC15_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC15_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC15_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&self) -> PC12_R {
        PC12_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&self) -> PC13_R {
        PC13_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&self) -> PC14_R {
        PC14_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&self) -> PC15_R {
        PC15_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&mut self) -> PC12_W {
        PC12_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&mut self) -> PC13_W {
        PC13_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&mut self) -> PC14_W {
        PC14_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&mut self) -> PC15_W {
        PC15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 0 Input/Output Control Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr12](index.html) module"]
pub struct IOCR12_SPEC;
impl crate::RegisterSpec for IOCR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocr12::R](R) reader structure"]
impl crate::Readable for IOCR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocr12::W](W) writer structure"]
impl crate::Writable for IOCR12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCR12 to value 0"]
impl crate::Resettable for IOCR12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
