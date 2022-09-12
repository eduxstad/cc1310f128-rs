# [doc = "Register `FSM_ACC_EP` reader"] pub struct R (crate :: R < FSM_ACC_EP_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < FSM_ACC_EP_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < FSM_ACC_EP_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < FSM_ACC_EP_SPEC >) -> Self { R (reader) } } # [doc = "Register `FSM_ACC_EP` writer"] pub struct W (crate :: W < FSM_ACC_EP_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < FSM_ACC_EP_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < FSM_ACC_EP_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < FSM_ACC_EP_SPEC >) -> Self { W (writer) } } # [doc = "Field `ACC_EP` reader - 15:0\\]
Internal. Only to be used through TI provided API."] pub type ACC_EP_R = crate :: FieldReader < u16 , u16 > ; # [doc = "Field `ACC_EP` writer - 15:0\\]
Internal. Only to be used through TI provided API."] pub type ACC_EP_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , FSM_ACC_EP_SPEC , u16 , u16 , 16 , O > ; # [doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."] pub type RESERVED16_R = crate :: FieldReader < u16 , u16 > ; # [doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."] pub type RESERVED16_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , FSM_ACC_EP_SPEC , u16 , u16 , 16 , O > ; impl R { # [doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."] # [inline (always)] pub fn acc_ep (& self) -> ACC_EP_R { ACC_EP_R :: new ((self . bits & 0xffff) as u16) } # [doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."] # [inline (always)] pub fn reserved16 (& self) -> RESERVED16_R { RESERVED16_R :: new (((self . bits >> 16) & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."] # [inline (always)] pub fn acc_ep (& mut self) -> ACC_EP_W < 0 > { ACC_EP_W :: new (self) } # [doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."] # [inline (always)] pub fn reserved16 (& mut self) -> RESERVED16_W < 16 > { RESERVED16_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_acc_ep](index.html) module"] pub struct FSM_ACC_EP_SPEC ; impl crate :: RegisterSpec for FSM_ACC_EP_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [fsm_acc_ep::R](R) reader structure"] impl crate :: Readable for FSM_ACC_EP_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [fsm_acc_ep::W](W) writer structure"] impl crate :: Writable for FSM_ACC_EP_SPEC { type Writer = W ; } # [doc = "`reset()` method sets FSM_ACC_EP to value 0"] impl crate :: Resettable for FSM_ACC_EP_SPEC { # [inline (always)] fn reset_value () -> Self :: Ux { 0 } }