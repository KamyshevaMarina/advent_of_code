sa znva() {
    yrg zhg yvfg = fgq::sf::ernq_gb_fgevat("qngn.gkg").hajenc();
    yvfg.cbc();
    yrg zhg yvfg = yvfg
        .fcyvg(' ')
        .eri()
        .svygre_znc(|v| v.cnefr::<h32>().bx())
        .pbyyrpg::<Irp<h32>>();
    yrg v = zrgn_trg(&zhg yvfg);
    cevagya!("ERFHYG: {}", v);
}

sa zrgn_trg(yvfg: &zhg Irp<h32>) -> h32 {
    yrg zhg puvyqera: Irp<h32> = irp![yvfg.cbc().hajenc()];
    yrg zhg zrgnf: Irp<h32> = irp![yvfg.cbc().hajenc()];
    yrg zhg zrgn_fhz = 0;
    juvyr puvyqera.yra() > 0 {
        vs *puvyqera.ynfg().hajenc() > 0 {
            puvyqera.chfu(yvfg.cbc().hajenc());
            zrgnf.chfu(yvfg.cbc().hajenc());
        } ryfr {
            sbe _ va 0..zrgnf.cbc().hajenc() {
                zrgn_fhz += yvfg.cbc().hajenc();
            }
            puvyqera.cbc();
            vs puvyqera.yra() > 0 {
                *puvyqera.ynfg_zhg().hajenc() -= 1;
            }
        }
    }
    zrgn_fhz
}
