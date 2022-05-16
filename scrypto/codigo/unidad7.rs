use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct DatosAfiliado {
    nombre: String
}

blueprint! {
    struct Afiliacion {
        badge: Vault,
        direccion_nft: ResourceAddress,
        num_afiliado: u32
    }

    impl Afiliacion {

        pub fn instantiate_dao() -> ComponentAddress {
            
            let badge = ResourceBuilder::new_fungible()
                .metadata("name", "Autorización mintear nuevos NFT")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);
                
            let afiliado = ResourceBuilder::new_non_fungible()
                .metadata("name", "Afiliado DAO")
                .mintable(rule!(require(badge.resource_address())), LOCKED)
                .no_initial_supply();

            Self {
                badge: Vault::with_bucket(badge),
                direccion_nft: afiliado,
                num_afiliado: 0
            }
            .instantiate()
            .globalize()
        }

        pub fn afiliarse_dao(&mut self, nombre: String) -> Bucket {
            self.num_afiliado += 1;
        
            self.badge.authorize(||{
                let resource_manager = borrow_resource_manager!(self.direccion_nft);
                    resource_manager.mint_non_fungible(
                        &NonFungibleId::from_u32(self.num_afiliado),
                        DatosAfiliado {
                            nombre: nombre
                        }
                    )
            })
        }
    }
}