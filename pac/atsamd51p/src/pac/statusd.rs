#[doc = "Register `STATUSD` reader"]
pub type R = crate::R<STATUSD_SPEC>;
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Protect Enable"]
pub type SERCOM4__R = crate::BitReader;
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Protect Enable"]
pub type SERCOM5__R = crate::BitReader;
#[doc = "Field `SERCOM6_` reader - SERCOM6 APB Protect Enable"]
pub type SERCOM6__R = crate::BitReader;
#[doc = "Field `SERCOM7_` reader - SERCOM7 APB Protect Enable"]
pub type SERCOM7__R = crate::BitReader;
#[doc = "Field `TCC4_` reader - TCC4 APB Protect Enable"]
pub type TCC4__R = crate::BitReader;
#[doc = "Field `TC6_` reader - TC6 APB Protect Enable"]
pub type TC6__R = crate::BitReader;
#[doc = "Field `TC7_` reader - TC7 APB Protect Enable"]
pub type TC7__R = crate::BitReader;
#[doc = "Field `ADC0_` reader - ADC0 APB Protect Enable"]
pub type ADC0__R = crate::BitReader;
#[doc = "Field `ADC1_` reader - ADC1 APB Protect Enable"]
pub type ADC1__R = crate::BitReader;
#[doc = "Field `DAC_` reader - DAC APB Protect Enable"]
pub type DAC__R = crate::BitReader;
#[doc = "Field `I2S_` reader - I2S APB Protect Enable"]
pub type I2S__R = crate::BitReader;
#[doc = "Field `PCC_` reader - PCC APB Protect Enable"]
pub type PCC__R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SERCOM4 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM6 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom6_(&self) -> SERCOM6__R {
        SERCOM6__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM7 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom7_(&self) -> SERCOM7__R {
        SERCOM7__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCC4 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc4_(&self) -> TCC4__R {
        TCC4__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC6 APB Protect Enable"]
    #[inline(always)]
    pub fn tc6_(&self) -> TC6__R {
        TC6__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC7 APB Protect Enable"]
    #[inline(always)]
    pub fn tc7_(&self) -> TC7__R {
        TC7__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Protect Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 APB Protect Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> ADC1__R {
        ADC1__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC APB Protect Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2S APB Protect Enable"]
    #[inline(always)]
    pub fn i2s_(&self) -> I2S__R {
        I2S__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCC APB Protect Enable"]
    #[inline(always)]
    pub fn pcc_(&self) -> PCC__R {
        PCC__R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSD_SPEC;
impl crate::RegisterSpec for STATUSD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusd::R`](R) reader structure"]
impl crate::Readable for STATUSD_SPEC {}
#[doc = "`reset()` method sets STATUSD to value 0"]
impl crate::Resettable for STATUSD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
