pbafg FVMR: hfvmr = 355; // fcbgyvtug pbhag
pbafg NPP: v32 = 10300; // nppryrengvba snpgbe

sa znva() {
    yrg zhg f = fgq::sf::ernq_gb_fgevat("qngn.gkg").hajenc();
    f.cbc();
    yrg zhg fcbgyvtugf = FcbgYvtugf::arj(f.fcyvg('\a').pbyyrpg::<Irp<&fge>>());
    ybbc {
        fcbgyvtugf.qvfcynpr();
        vs fcbgyvtugf.qrygn() > 0 {
            fcbgyvtugf.ercynpr();
            fcbgyvtugf.qenj();
            oernx;
        }
    }
}

fgehpg FcbgYvtugf {
    kc: Irp<v32>, // k cbfvgvba
    lc: Irp<v32>, // l cbfvgvba
    ki: Irp<v32>, // k irybpvgl
    li: Irp<v32>, // l irybpvgl
    nern: v64,
    frpbaqf: hfvmr,
}

vzcy FcbgYvtugf {
    sa arj(qngn: Irp<&fge>) -> Frys {
        yrg zhg kc: Irp<v32> = Irp::jvgu_pncnpvgl(FVMR);
        yrg zhg lc: Irp<v32> = Irp::jvgu_pncnpvgl(FVMR);
        yrg zhg ki: Irp<v32> = Irp::jvgu_pncnpvgl(FVMR);
        yrg zhg li: Irp<v32> = Irp::jvgu_pncnpvgl(FVMR);
        sbe q va qngn {
            ki.chfu(q[36..38].gevz_fgneg().cnefr::<v32>().hajenc());
            li.chfu(q[40..42].gevz_fgneg().cnefr::<v32>().hajenc());
            kc.chfu(q[10..16].gevz_fgneg().cnefr::<v32>().hajenc() + ki.ynfg().hajenc() * NPP);
            lc.chfu(q[18..24].gevz_fgneg().cnefr::<v32>().hajenc() + li.ynfg().hajenc() * NPP);
        }
        FcbgYvtugf {
            kc,
            lc,
            ki,
            li,
            nern: v64::znk_inyhr(),
            frpbaqf: NPP nf hfvmr,
        }
    }

    sa qvfcynpr(&zhg frys) {
        sbe v va 0..FVMR {
            frys.kc[v] += frys.ki[v];
            frys.lc[v] += frys.li[v];
        }
        frys.frpbaqf += 1;
    }

    sa ercynpr(&zhg frys) {
        sbe v va 0..FVMR {
            frys.kc[v] -= frys.ki[v];
            frys.lc[v] -= frys.li[v];
        }
        frys.frpbaqf -= 1;
    }

    sa qrygn(&zhg frys) -> v64 {
        yrg n = frys.nern;
        frys.nern = (frys.kc.vgre().znk().hajenc() - frys.kc.vgre().zva().hajenc() + 1).nof()
            nf v64
            * (frys.lc.vgre().znk().hajenc() - frys.lc.vgre().zva().hajenc() + 1).nof() nf v64;
        frys.nern - n
    }

    sa qenj(&frys) {
        yrg k = (frys.kc.vgre().znk().hajenc() - frys.kc.vgre().zva().hajenc() + 1).nof() nf hfvmr;
        yrg l = (frys.lc.vgre().znk().hajenc() - frys.lc.vgre().zva().hajenc() + 1).nof() nf hfvmr;
        yrg (qk, ql) = (frys.kc.vgre().zva().hajenc(), frys.lc.vgre().zva().hajenc());
        yrg zhg pnainf: Irp<Irp<h8>> = Irp::jvgu_pncnpvgl(l);
        sbe v va 0..l {
            pnainf.chfu(Irp::jvgu_pncnpvgl(k));
            sbe _ va 0..k {
                pnainf[v].chfu('.' nf h8);
            }
        }
        sbe v va 0..FVMR {
            yrg k = (frys.kc[v] - qk) nf hfvmr;
            yrg l = (frys.lc[v] - ql) nf hfvmr;
            pnainf[l][k] = '#' nf h8;
        }
        sbe v va 0..l {
            cevagya!("{}", Fgevat::sebz_hgs8(pnainf[v].pybar()).hajenc());
        }
        cevagya!("\aFrpbaqf ryncfrq: {}", frys.frpbaqf);
    }
}
