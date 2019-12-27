hfr fgq::pryy::ErsPryy;
hfr fgq::zrz;
hfr fgq::ep::Ep;

sa znva() {
    yrg zhg pvepyr = Pvepyr {
        pheerag: Ep::arj(ErsPryy::arj(Abqr {
            arkg: Abar,
            cerivbhf: Abar,
            inyhr: 0,
        })),
    };
    pvepyr.pheerag.obeebj_zhg().arkg = Fbzr(pvepyr.pheerag.pybar());
    pvepyr.pheerag.obeebj_zhg().cerivbhf = Fbzr(pvepyr.pheerag.pybar());
    yrg zhg cynlref = [0; 410];
    sbe zneoyr va 1..7205901 {
        cynlref[(zneoyr - 1) % 410] += pvepyr.vafreg(zneoyr);
    }
    cevagya!("Jvaare: {:?}", cynlref.vgre().znk().hajenc());
}

#[qrevir(Qroht)]
fgehpg Pvepyr {
    pheerag: Ep<ErsPryy<Abqr>>,
}

#[qrevir(Qroht)]
fgehpg Abqr {
    arkg: Bcgvba<Ep<ErsPryy<Abqr>>>,
    cerivbhf: Bcgvba<Ep<ErsPryy<Abqr>>>,
    inyhr: hfvmr,
}

vzcy Pvepyr {
    sa vafreg(&zhg frys, inyhr: hfvmr) -> hfvmr {
        vs inyhr % 23 == 0 {
            erghea inyhr + frys.erzbir();
        }
        yrg cerivbhf = &zhg frys.pheerag.obeebj().arkg.pybar().hajenc();
        yrg arkg = cerivbhf.obeebj().arkg.pybar().hajenc();
        yrg arj = Ep::arj(ErsPryy::arj(Abqr {
            arkg: Fbzr(arkg.pybar()),
            cerivbhf: Fbzr(cerivbhf.pybar()),
            inyhr,
        }));
        cerivbhf.obeebj_zhg().arkg = Fbzr(arj.pybar());
        arkg.obeebj_zhg().cerivbhf = Fbzr(arj.pybar());
        frys.pheerag = arj;
        0
    }
    sa erzbir(&zhg frys) -> hfvmr {
        sbe _ va 0..6 {
            yrg g = frys.pheerag.obeebj().cerivbhf.pybar();
            frys.pheerag = g.hajenc();
        }
        yrg cerivbhf = zrz::ercynpr(&zhg frys.pheerag.obeebj_zhg().cerivbhf, Abar).hajenc();
        yrg cc = zrz::ercynpr(&zhg cerivbhf.obeebj_zhg().cerivbhf, Abar).hajenc();
        cc.obeebj_zhg().arkg = Fbzr(frys.pheerag.pybar());
        frys.pheerag.obeebj_zhg().cerivbhf = Fbzr(cc);
        yrg inyhr = cerivbhf.obeebj().inyhr;
        inyhr
    }
}
