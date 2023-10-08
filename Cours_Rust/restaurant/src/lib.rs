pub use crate::salle_a_manger::accueil;

pub fn manger_au_restaurant() {
    // On commande un petit-déjeuner en été avec tartine grillée au seigle
    let mut repas = cuisines::PetitDejeuner::en_ete("seigle");
    // On change d'avis sur le pain que nous souhaitons
    repas.tartine_grillee = String::from("blé");
    println!( "Je voudrais une tartine grillée au {}, s'il vous plaît.",
              repas.tartine_grillee);
    
    // La prochaine ligne ne va pas se compiler si nous ne la commentons pas,
    // car nous ne sommes pas autorisés à voir ou modifier le fruit de saison
    // qui accompagne le repas.

    // repas.fruit_de_saison = String::from("myrtilles");
}

fn servir_commande() {}

mod cuisines {
    fn corriger_commande_erronee() {
        cuisiner_commande();
        super::servir_commande();
    }

    impl PetitDejeuner {
        pub fn en_ete(tartine_grillee: &str) -> PetitDejeuner {
            PetitDejeuner {
                tartine_grillee: String::from(tartine_grillee),
                fruit_de_saison: String::from("pêches"),
            }
        }
    }

    fn cuisiner_commande() {}
}
