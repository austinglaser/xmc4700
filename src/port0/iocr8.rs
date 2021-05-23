#[doc = "Register `IOCR8` reader"]
pub struct R(crate::R<IOCR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IOCR8_SPEC>> for R {
    fn from(reader: crate::R<IOCR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCR8` writer"]
pub struct W(crate::W<IOCR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCR8_SPEC>;
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
impl core::convert::From<crate::W<IOCR8_SPEC>> for W {
    fn from(writer: crate::W<IOCR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Control for Port n Pin 8 to 11"]
pub type PC8_A = super::iocr0::PC0_A;
#[doc = "Field `PC8` reader - Port Control for Port n Pin 8 to 11"]
pub type PC8_R = super::iocr0::PC0_R;
#[doc = "Field `PC8` writer - Port Control for Port n Pin 8 to 11"]
pub struct PC8_W<'a> {
    w: &'a mut W,
}
impl<'a> PC8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC8_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC8_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC8_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC8_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC8_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC8_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC8_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC8_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC8_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC8_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC8_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC8_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC8_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC8_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC8_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC8_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC8_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC8_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 8 to 11"]
pub type PC9_A = super::iocr0::PC0_A;
#[doc = "Field `PC9` reader - Port Control for Port n Pin 8 to 11"]
pub type PC9_R = super::iocr0::PC0_R;
#[doc = "Field `PC9` writer - Port Control for Port n Pin 8 to 11"]
pub struct PC9_W<'a> {
    w: &'a mut W,
}
impl<'a> PC9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC9_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC9_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC9_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC9_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC9_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC9_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC9_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC9_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC9_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC9_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC9_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC9_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC9_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC9_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC9_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC9_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC9_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC9_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 8 to 11"]
pub type PC10_A = super::iocr0::PC0_A;
#[doc = "Field `PC10` reader - Port Control for Port n Pin 8 to 11"]
pub type PC10_R = super::iocr0::PC0_R;
#[doc = "Field `PC10` writer - Port Control for Port n Pin 8 to 11"]
pub struct PC10_W<'a> {
    w: &'a mut W,
}
impl<'a> PC10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC10_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC10_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC10_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC10_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC10_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC10_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC10_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC10_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC10_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC10_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC10_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC10_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC10_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC10_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC10_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC10_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC10_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC10_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 8 to 11"]
pub type PC11_A = super::iocr0::PC0_A;
#[doc = "Field `PC11` reader - Port Control for Port n Pin 8 to 11"]
pub type PC11_R = super::iocr0::PC0_R;
#[doc = "Field `PC11` writer - Port Control for Port n Pin 8 to 11"]
pub struct PC11_W<'a> {
    w: &'a mut W,
}
impl<'a> PC11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC11_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC11_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC11_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC11_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC11_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC11_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC11_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC11_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC11_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC11_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC11_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC11_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC11_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC11_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC11_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC11_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC11_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC11_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc8(&self) -> PC8_R {
        PC8_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc9(&self) -> PC9_R {
        PC9_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc10(&self) -> PC10_R {
        PC10_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc11(&self) -> PC11_R {
        PC11_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc8(&mut self) -> PC8_W {
        PC8_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc9(&mut self) -> PC9_W {
        PC9_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc10(&mut self) -> PC10_W {
        PC10_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc11(&mut self) -> PC11_W {
        PC11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 0 Input/Output Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr8](index.html) module"]
pub struct IOCR8_SPEC;
impl crate::RegisterSpec for IOCR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocr8::R](R) reader structure"]
impl crate::Readable for IOCR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocr8::W](W) writer structure"]
impl crate::Writable for IOCR8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCR8 to value 0"]
impl crate::Resettable for IOCR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
