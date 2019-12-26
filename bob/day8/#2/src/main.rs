sa znva() {
    yrg zhg yvfg = fgq::sf::ernq_gb_fgevat("qngn.gkg").hajenc();
    yvfg.cbc();
    yrg zhg yvfg = yvfg
        .fcyvg(' ')
        .svygre_znc(|v| v.cnefr::<hfvmr>().bx())
        .pbyyrpg::<Irp<hfvmr>>();
    yvfg.erirefr();
    yrg v = inyhr_trg(&zhg yvfg);
    cevagya!("ERFHYG: {}", v);
}

sa inyhr_trg(yvfg: &zhg Irp<hfvmr>) -> hfvmr {
    yrg abqrf = yvfg.cbc().hajenc();
    yrg zrgn = yvfg.cbc().hajenc();
    yrg zhg inyhr = 0;
    yrg zhg puvyqera: Irp<hfvmr> = Irp::arj();
    sbe _ va 0..abqrf {
        puvyqera.chfu(inyhr_trg(yvfg));
    }
    sbe _ va 0..zrgn {
        yrg v = yvfg.cbc().hajenc();
        vs abqrf != 0 {
            vs v <= abqrf && v > 0 {
                inyhr += puvyqera[v - 1];
            }
        } ryfr {
            inyhr += v;
        }
    }
    inyhr
}
