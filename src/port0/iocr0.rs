#[doc = "Register `IOCR0` reader"]
pub struct R(crate::R<IOCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IOCR0_SPEC>> for R {
    fn from(reader: crate::R<IOCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCR0` writer"]
pub struct W(crate::W<IOCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCR0_SPEC>;
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
impl core::convert::From<crate::W<IOCR0_SPEC>> for W {
    fn from(writer: crate::W<IOCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC0_A {
    #[doc = "0: Input - No internal pull device active"]
    INPUT = 0,
    #[doc = "1: Input - Internal pull-down device active"]
    INPUT_PULLDOWN = 1,
    #[doc = "2: Input - Internal pull-up device active"]
    INPUT_PULLUP = 2,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    INPUT_SAMPLED = 3,
    #[doc = "4: Input inverted - No internal pull device active"]
    INPUT_INV = 4,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    INPUT_INV_PULLDOWN = 5,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    INPUT_INV_PULLUP = 6,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    INPUT_INV_SAMPLED = 7,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    OUTPUT = 16,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    OUTPUT_AF1 = 17,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    OUTPUT_AF2 = 18,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    OUTPUT_AF3 = 19,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    OUTPUT_AF4 = 20,
    #[doc = "24: Output Open Drain - General-purpose output"]
    OUTPUT_OD = 24,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    OUTPUT_OD_AF1 = 25,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    OUTPUT_OD_AF2 = 26,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    OUTPUT_OD_AF3 = 27,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    OUTPUT_OD_AF4 = 28,
}
impl From<PC0_A> for u8 {
    #[inline(always)]
    fn from(variant: PC0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC0` reader - Port Control for Port n Pin 0 to 3"]
pub struct PC0_R(crate::FieldReader<u8, PC0_A>);
impl PC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC0_A> {
        match self.bits {
            0 => Some(PC0_A::INPUT),
            1 => Some(PC0_A::INPUT_PULLDOWN),
            2 => Some(PC0_A::INPUT_PULLUP),
            3 => Some(PC0_A::INPUT_SAMPLED),
            4 => Some(PC0_A::INPUT_INV),
            5 => Some(PC0_A::INPUT_INV_PULLDOWN),
            6 => Some(PC0_A::INPUT_INV_PULLUP),
            7 => Some(PC0_A::INPUT_INV_SAMPLED),
            16 => Some(PC0_A::OUTPUT),
            17 => Some(PC0_A::OUTPUT_AF1),
            18 => Some(PC0_A::OUTPUT_AF2),
            19 => Some(PC0_A::OUTPUT_AF3),
            20 => Some(PC0_A::OUTPUT_AF4),
            24 => Some(PC0_A::OUTPUT_OD),
            25 => Some(PC0_A::OUTPUT_OD_AF1),
            26 => Some(PC0_A::OUTPUT_OD_AF2),
            27 => Some(PC0_A::OUTPUT_OD_AF3),
            28 => Some(PC0_A::OUTPUT_OD_AF4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PC0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUT_PULLDOWN`"]
    #[inline(always)]
    pub fn is_input_pulldown(&self) -> bool {
        **self == PC0_A::INPUT_PULLDOWN
    }
    #[doc = "Checks if the value of the field is `INPUT_PULLUP`"]
    #[inline(always)]
    pub fn is_input_pullup(&self) -> bool {
        **self == PC0_A::INPUT_PULLUP
    }
    #[doc = "Checks if the value of the field is `INPUT_SAMPLED`"]
    #[inline(always)]
    pub fn is_input_sampled(&self) -> bool {
        **self == PC0_A::INPUT_SAMPLED
    }
    #[doc = "Checks if the value of the field is `INPUT_INV`"]
    #[inline(always)]
    pub fn is_input_inv(&self) -> bool {
        **self == PC0_A::INPUT_INV
    }
    #[doc = "Checks if the value of the field is `INPUT_INV_PULLDOWN`"]
    #[inline(always)]
    pub fn is_input_inv_pulldown(&self) -> bool {
        **self == PC0_A::INPUT_INV_PULLDOWN
    }
    #[doc = "Checks if the value of the field is `INPUT_INV_PULLUP`"]
    #[inline(always)]
    pub fn is_input_inv_pullup(&self) -> bool {
        **self == PC0_A::INPUT_INV_PULLUP
    }
    #[doc = "Checks if the value of the field is `INPUT_INV_SAMPLED`"]
    #[inline(always)]
    pub fn is_input_inv_sampled(&self) -> bool {
        **self == PC0_A::INPUT_INV_SAMPLED
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PC0_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT_AF1`"]
    #[inline(always)]
    pub fn is_output_af1(&self) -> bool {
        **self == PC0_A::OUTPUT_AF1
    }
    #[doc = "Checks if the value of the field is `OUTPUT_AF2`"]
    #[inline(always)]
    pub fn is_output_af2(&self) -> bool {
        **self == PC0_A::OUTPUT_AF2
    }
    #[doc = "Checks if the value of the field is `OUTPUT_AF3`"]
    #[inline(always)]
    pub fn is_output_af3(&self) -> bool {
        **self == PC0_A::OUTPUT_AF3
    }
    #[doc = "Checks if the value of the field is `OUTPUT_AF4`"]
    #[inline(always)]
    pub fn is_output_af4(&self) -> bool {
        **self == PC0_A::OUTPUT_AF4
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OD`"]
    #[inline(always)]
    pub fn is_output_od(&self) -> bool {
        **self == PC0_A::OUTPUT_OD
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OD_AF1`"]
    #[inline(always)]
    pub fn is_output_od_af1(&self) -> bool {
        **self == PC0_A::OUTPUT_OD_AF1
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OD_AF2`"]
    #[inline(always)]
    pub fn is_output_od_af2(&self) -> bool {
        **self == PC0_A::OUTPUT_OD_AF2
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OD_AF3`"]
    #[inline(always)]
    pub fn is_output_od_af3(&self) -> bool {
        **self == PC0_A::OUTPUT_OD_AF3
    }
    #[doc = "Checks if the value of the field is `OUTPUT_OD_AF4`"]
    #[inline(always)]
    pub fn is_output_od_af4(&self) -> bool {
        **self == PC0_A::OUTPUT_OD_AF4
    }
}
impl core::ops::Deref for PC0_R {
    type Target = crate::FieldReader<u8, PC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC0` writer - Port Control for Port n Pin 0 to 3"]
pub struct PC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC0_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC0_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC0_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC0_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC0_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC0_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC0_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC0_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC0_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC0_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC0_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC0_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC0_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC0_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC0_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC0_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC0_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC0_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 0 to 3"]
pub type PC1_A = PC0_A;
#[doc = "Field `PC1` reader - Port Control for Port n Pin 0 to 3"]
pub type PC1_R = PC0_R;
#[doc = "Field `PC1` writer - Port Control for Port n Pin 0 to 3"]
pub struct PC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC1_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC1_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC1_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC1_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC1_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC1_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC1_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC1_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC1_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC1_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC1_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC1_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC1_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC1_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC1_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC1_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC1_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC1_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 0 to 3"]
pub type PC2_A = PC0_A;
#[doc = "Field `PC2` reader - Port Control for Port n Pin 0 to 3"]
pub type PC2_R = PC0_R;
#[doc = "Field `PC2` writer - Port Control for Port n Pin 0 to 3"]
pub struct PC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC2_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC2_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC2_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC2_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC2_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC2_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC2_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC2_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC2_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC2_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC2_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC2_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC2_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC2_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC2_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC2_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC2_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC2_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 0 to 3"]
pub type PC3_A = PC0_A;
#[doc = "Field `PC3` reader - Port Control for Port n Pin 0 to 3"]
pub type PC3_R = PC0_R;
#[doc = "Field `PC3` writer - Port Control for Port n Pin 0 to 3"]
pub struct PC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PC3_A::INPUT)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_pulldown(self) -> &'a mut W {
        self.variant(PC3_A::INPUT_PULLDOWN)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_pullup(self) -> &'a mut W {
        self.variant(PC3_A::INPUT_PULLUP)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_sampled(self) -> &'a mut W {
        self.variant(PC3_A::INPUT_SAMPLED)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn input_inv(self) -> &'a mut W {
        self.variant(PC3_A::INPUT_INV)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn input_inv_pulldown(self) -> &'a mut W {
        self.variant(PC3_A::INPUT_INV_PULLDOWN)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn input_inv_pullup(self) -> &'a mut W {
        self.variant(PC3_A::INPUT_INV_PULLUP)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn input_inv_sampled(self) -> &'a mut W {
        self.variant(PC3_A::INPUT_INV_SAMPLED)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PC3_A::OUTPUT)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn output_af1(self) -> &'a mut W {
        self.variant(PC3_A::OUTPUT_AF1)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn output_af2(self) -> &'a mut W {
        self.variant(PC3_A::OUTPUT_AF2)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn output_af3(self) -> &'a mut W {
        self.variant(PC3_A::OUTPUT_AF3)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn output_af4(self) -> &'a mut W {
        self.variant(PC3_A::OUTPUT_AF4)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn output_od(self) -> &'a mut W {
        self.variant(PC3_A::OUTPUT_OD)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn output_od_af1(self) -> &'a mut W {
        self.variant(PC3_A::OUTPUT_OD_AF1)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn output_od_af2(self) -> &'a mut W {
        self.variant(PC3_A::OUTPUT_OD_AF2)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn output_od_af3(self) -> &'a mut W {
        self.variant(PC3_A::OUTPUT_OD_AF3)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn output_od_af4(self) -> &'a mut W {
        self.variant(PC3_A::OUTPUT_OD_AF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&self) -> PC0_R {
        PC0_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&self) -> PC1_R {
        PC1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&self) -> PC2_R {
        PC2_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&self) -> PC3_R {
        PC3_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&mut self) -> PC0_W {
        PC0_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&mut self) -> PC1_W {
        PC1_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&mut self) -> PC2_W {
        PC2_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&mut self) -> PC3_W {
        PC3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 0 Input/Output Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr0](index.html) module"]
pub struct IOCR0_SPEC;
impl crate::RegisterSpec for IOCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocr0::R](R) reader structure"]
impl crate::Readable for IOCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocr0::W](W) writer structure"]
impl crate::Writable for IOCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCR0 to value 0"]
impl crate::Resettable for IOCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
