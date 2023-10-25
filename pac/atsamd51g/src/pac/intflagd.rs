#[doc = "Register `INTFLAGD` reader"]
pub type R = crate::R<INTFLAGD_SPEC>;
#[doc = "Register `INTFLAGD` writer"]
pub type W = crate::W<INTFLAGD_SPEC>;
#[doc = "Field `SERCOM4_` reader - SERCOM4"]
pub type SERCOM4__R = crate::BitReader;
#[doc = "Field `SERCOM4_` writer - SERCOM4"]
pub type SERCOM4__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERCOM5_` reader - SERCOM5"]
pub type SERCOM5__R = crate::BitReader;
#[doc = "Field `SERCOM5_` writer - SERCOM5"]
pub type SERCOM5__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC0_` reader - ADC0"]
pub type ADC0__R = crate::BitReader;
#[doc = "Field `ADC0_` writer - ADC0"]
pub type ADC0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1_` reader - ADC1"]
pub type ADC1__R = crate::BitReader;
#[doc = "Field `ADC1_` writer - ADC1"]
pub type ADC1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAC_` reader - DAC"]
pub type DAC__R = crate::BitReader;
#[doc = "Field `DAC_` writer - DAC"]
pub type DAC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCC_` reader - PCC"]
pub type PCC__R = crate::BitReader;
#[doc = "Field `PCC_` writer - PCC"]
pub type PCC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SERCOM4"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM5"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1"]
    #[inline(always)]
    pub fn adc1_(&self) -> ADC1__R {
        ADC1__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - PCC"]
    #[inline(always)]
    pub fn pcc_(&self) -> PCC__R {
        PCC__R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM4"]
    #[inline(always)]
    #[must_use]
    pub fn sercom4_(&mut self) -> SERCOM4__W<INTFLAGD_SPEC, 0> {
        SERCOM4__W::new(self)
    }
    #[doc = "Bit 1 - SERCOM5"]
    #[inline(always)]
    #[must_use]
    pub fn sercom5_(&mut self) -> SERCOM5__W<INTFLAGD_SPEC, 1> {
        SERCOM5__W::new(self)
    }
    #[doc = "Bit 7 - ADC0"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_(&mut self) -> ADC0__W<INTFLAGD_SPEC, 7> {
        ADC0__W::new(self)
    }
    #[doc = "Bit 8 - ADC1"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_(&mut self) -> ADC1__W<INTFLAGD_SPEC, 8> {
        ADC1__W::new(self)
    }
    #[doc = "Bit 9 - DAC"]
    #[inline(always)]
    #[must_use]
    pub fn dac_(&mut self) -> DAC__W<INTFLAGD_SPEC, 9> {
        DAC__W::new(self)
    }
    #[doc = "Bit 11 - PCC"]
    #[inline(always)]
    #[must_use]
    pub fn pcc_(&mut self) -> PCC__W<INTFLAGD_SPEC, 11> {
        PCC__W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflagd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflagd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAGD_SPEC;
impl crate::RegisterSpec for INTFLAGD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagd::R`](R) reader structure"]
impl crate::Readable for INTFLAGD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflagd::W`](W) writer structure"]
impl crate::Writable for INTFLAGD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGD to value 0"]
impl crate::Resettable for INTFLAGD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
