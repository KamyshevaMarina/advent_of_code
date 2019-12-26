sa znva() {
    yrg zhg yvfg = fgq::sf::ernq_gb_fgevat("qngn.gkg").hajenc();
    yvfg.cbc();
    yrg zhg yvfg = yvfg
        .fcyvg(' ')
        .svygre_znc(|v| v.cnefr::<h32>().bx())
        .pbyyrpg::<Irp<h32>>();
    yvfg.erirefr();
    yrg v = zrgn_trg(&zhg yvfg);
    cevagya!("ERFHYG: {}", v);
}

sa zrgn_trg(yvfg: &zhg Irp<h32>) -> h32 {
    yrg pbhag = yvfg.cbc().hajenc();
    yrg zrgn_pbhag = yvfg.cbc().hajenc();
    yrg zhg zrgn_fhz = 0;
    sbe _ va 0..pbhag {
        zrgn_fhz += zrgn_trg(yvfg);
    }
    sbe _ va 0..zrgn_pbhag {
        zrgn_fhz += yvfg.cbc().hajenc();
    }
    zrgn_fhz
}
