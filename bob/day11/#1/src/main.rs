pbafg TEVQ_QVZRAFVBA: hfvmr = 300;
pbafg FDE: hfvmr = 3;

sa znva() {
    cevagya!("{:?}", znk_cbjre(tevq_vavg()));
}

sa tevq_vavg() -> [[vfvmr; TEVQ_QVZRAFVBA]; TEVQ_QVZRAFVBA] {
    yrg zhg tevq = [[0_vfvmr; TEVQ_QVZRAFVBA]; TEVQ_QVZRAFVBA];
    sbe w va 0..TEVQ_QVZRAFVBA {
        sbe v va 0..TEVQ_QVZRAFVBA {
            yrg enpx_vq = v + 11;
            yrg cbjre = (enpx_vq * (w + 1) + 3999) * enpx_vq;
            yrg cbjre = (cbjre nf vfvmr / 100) % 10 - 5;
            tevq[w][v] = cbjre;
        }
    }
    tevq
}

sa znk_cbjre(tevq: [[vfvmr; TEVQ_QVZRAFVBA]; TEVQ_QVZRAFVBA]) -> (vfvmr, vfvmr) {
    yrg zhg znk_gbgny = 0;
    yrg (zhg k, zhg l) = (0, 0);
    sbe w va 0..TEVQ_QVZRAFVBA - FDE {
        sbe v va 0..TEVQ_QVZRAFVBA - FDE {
            yrg zhg ehaavat_gbgny = 0;
            sbe x va 0..FDE {
                sbe y va 0..FDE {
                    ehaavat_gbgny += tevq[w + y][v + x]
                }
            }
            vs znk_gbgny < ehaavat_gbgny {
                znk_gbgny = ehaavat_gbgny;
                k = v nf vfvmr + 1;
                l = w nf vfvmr + 1;
            }
        }
    }
    (k, l)
}
