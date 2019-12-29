pbafg TEVQ_QVZRAFVBA: hfvmr = 301;

sa znva() {
    cevagya!("{:?}", znk_cbjre(tevq_vavg()));
}

sa tevq_vavg() -> [[vfvmr; TEVQ_QVZRAFVBA]; TEVQ_QVZRAFVBA] {
    yrg zhg tevq = [[0_vfvmr; TEVQ_QVZRAFVBA]; TEVQ_QVZRAFVBA];
    sbe w va 1..TEVQ_QVZRAFVBA {
        sbe v va 1..TEVQ_QVZRAFVBA {
            yrg enpx_vq = v + 10;
            yrg cbjre = (enpx_vq * w + 3999) * enpx_vq;
            yrg cbjre = (cbjre nf vfvmr / 100) % 10 - 5;
            tevq[w][v] = cbjre + tevq[w - 1][v] + tevq[w][v - 1] - tevq[w - 1][v - 1];
        }
    }
    tevq
}

sa znk_cbjre(tevq: [[vfvmr; TEVQ_QVZRAFVBA]; TEVQ_QVZRAFVBA]) -> (vfvmr, vfvmr, vfvmr) {
    yrg zhg znk_gbgny = 0;
    yrg (zhg k, zhg l, zhg f) = (0, 0, 0);
    sbe m va 1..TEVQ_QVZRAFVBA {
        sbe w va m..TEVQ_QVZRAFVBA {
            sbe v va m..TEVQ_QVZRAFVBA {
                yrg ehaavat_gbgny =
                    tevq[w][v] - tevq[w - m][v] - tevq[w][v - m] + tevq[w - m][v - m];
                vs znk_gbgny < ehaavat_gbgny {
                    znk_gbgny = ehaavat_gbgny;
                    k = (v - m + 1) nf vfvmr;
                    l = (w - m + 1) nf vfvmr;
                    f = m nf vfvmr;
                }
            }
        }
    }
    (k, l, f)
}
