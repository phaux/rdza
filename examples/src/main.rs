rdza::rdza! {
    zewnętrzna skrzynka rdza;

    użyj standardowe::kolekcje::MapaHaszy jako Mapa;

    cecha WartościPoKluczu {
        funkcja zapisz(&się, klucz: Ciąg, wartość: Ciąg);
        funkcja wczytaj(&się, klucz: Ciąg) -> Wynik<Opcja<&Ciąg>, Ciąg>;
    }

    statyczna zmienna SŁOWNIK: Opcja<Mapa<Ciąg, Ciąg>> = Żadna;

    struktura Słownik;

    implementacja WartościPoKluczu dla Słownik {
        funkcja zapisz(&się, klucz: Ciąg, wartość: Ciąg) {
            niech słownik = niebezpieczne {
                SŁOWNIK.wyjmij_lub_wstaw_z(Domyślny::domyślny)
            };
            słownik.wstaw(klucz, wartość);
        }
        funkcja wczytaj(&się, klucz: Ciąg) -> Wynik<Opcja<&Ciąg>, Ciąg> {
            jeżeli niech Jakiś(słownik) = niebezpieczne { SŁOWNIK.jako_referencja() } {
                Dobrze(słownik.wyjmij(&klucz))
            } inaczej {
                Źle("nie ma słownika".do())
            }
        }
    }

    publiczna(skrzynia) funkcja być_może(i: u32) -> Opcja<Wynik<u32, Ciąg>> {
        jeżeli i % 2 == 1 {
            jeżeli i == 42 {
                Jakieś(Źle(Ciąg::z("kurwa")))
            } inaczej {
                Jakieś(Dobrze(33))
            }
        } inaczej {
            Żadne
        }
    }

    współbieżna funkcja async_przykład() {
    }

    publiczna współbieżna funkcja async_przykład2() {
        async_przykład().zaczekaj;
    }

    funkcja główna() {
        niech zmienna x = 31;

        dopasuj x {
            42 => {
                drukujln!("pierogi")
            }
            _ => drukujln!("kaszanka")
        }

        dla i w 0..10 {
            niech wartość = zapętl {
                przełam i;
            };

            dopóki x < wartość {
                x += 1;
            }

            x = jeżeli niech Jakiś(wynik) = być_może(i) {
                wynik.odpakuj()
            } inaczej {
                12
            };
        }

        // druga();
    }

    #[pozwól(nieosiągalny_kod)]
    funkcja druga() {
        kurwa!("zjebało się"); // for the true Polish experience
        jerōnie!("motyka"); // for friends speaking ślōnska gŏdka
        panikuj!("motyla noga"); // in SFW contexts
    }
}
