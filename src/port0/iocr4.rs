#[doc = "Register `IOCR4` reader"]
pub struct R(crate::R<IOCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IOCR4_SPEC>> for R {
    fn from(reader: crate::R<IOCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCR4` writer"]
pub struct W(crate::W<IOCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCR4_SPEC>;
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
impl core::convert::From<crate::W<IOCR4_SPEC>> for W {
    fn from(writer: crate::W<IOCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Control for Port n Pin 4 to 7"]
pub type PC4_A = super::iocr0::PC0_A;
#[doc = "Field `PC4` reader - Port Control for Port n Pin 4 to 7"]
pub type PC4_R = super::iocr0::PC0_R;
#[doc = "Field `PC4` writer - Port Control for Port n Pin 4 to 7"]
pub struct PC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC4_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC4_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC4_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC4_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC4_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC4_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC4_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC4_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC4_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC4_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC4_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC4_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC4_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC4_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC4_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC4_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC4_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC4_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 4 to 7"]
pub type PC5_A = super::iocr0::PC0_A;
#[doc = "Field `PC5` reader - Port Control for Port n Pin 4 to 7"]
pub type PC5_R = super::iocr0::PC0_R;
#[doc = "Field `PC5` writer - Port Control for Port n Pin 4 to 7"]
pub struct PC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PC5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC5_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC5_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC5_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC5_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC5_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC5_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC5_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC5_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC5_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC5_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC5_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC5_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC5_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC5_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC5_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC5_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC5_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC5_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 4 to 7"]
pub type PC6_A = super::iocr0::PC0_A;
#[doc = "Field `PC6` reader - Port Control for Port n Pin 4 to 7"]
pub type PC6_R = super::iocr0::PC0_R;
#[doc = "Field `PC6` writer - Port Control for Port n Pin 4 to 7"]
pub struct PC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PC6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC6_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC6_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC6_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC6_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC6_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC6_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC6_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC6_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC6_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC6_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC6_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC6_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC6_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC6_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC6_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC6_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC6_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC6_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 4 to 7"]
pub type PC7_A = super::iocr0::PC0_A;
#[doc = "Field `PC7` reader - Port Control for Port n Pin 4 to 7"]
pub type PC7_R = super::iocr0::PC0_R;
#[doc = "Field `PC7` writer - Port Control for Port n Pin 4 to 7"]
pub struct PC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PC7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC7_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC7_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC7_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC7_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC7_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC7_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC7_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC7_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC7_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC7_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC7_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC7_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC7_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC7_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC7_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC7_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC7_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC7_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&self) -> PC4_R {
        PC4_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&self) -> PC5_R {
        PC5_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&self) -> PC7_R {
        PC7_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&mut self) -> PC4_W {
        PC4_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&mut self) -> PC5_W {
        PC5_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&mut self) -> PC6_W {
        PC6_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&mut self) -> PC7_W {
        PC7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 0 Input/Output Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr4](index.html) module"]
pub struct IOCR4_SPEC;
impl crate::RegisterSpec for IOCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocr4::R](R) reader structure"]
impl crate::Readable for IOCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocr4::W](W) writer structure"]
impl crate::Writable for IOCR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCR4 to value 0"]
impl crate::Resettable for IOCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
