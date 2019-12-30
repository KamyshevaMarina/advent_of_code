hfr fgq::pbyyrpgvbaf::UnfuFrg;

sa znva() {
    yrg zhg f = fgq::sf::ernq_gb_fgevat("qngn.gkg")
        .hajenc()
        .ercynpr('.', " ");
    f.cbc();
    yrg zhg f = f.fcyvg('\a').pbyyrpg::<Irp<&fge>>();

    yrg zhg cynagrq: UnfuFrg<&[h8]> = UnfuFrg::arj();
    juvyr yrg Fbzr(y) = f.cbc() {
        vs y.vf_rzcgl() {
            oernx;
        }
        vs yrg 0k23 = &y.nf_olgrf()[9] {
            cynagrq.vafreg(&y.nf_olgrf()[0..5]);
        }
    }
    yrg zhg fgngr = "    ".gb_fgevat();
    fgngr = fgngr + &f.erzbir(0)[15..];
    fgngr = fgngr + &"    ".gb_fgevat();
    yrg zhg cnggreaf: UnfuFrg<Fgevat> = UnfuFrg::arj();
    cnggreaf.vafreg(fgngr.pybar().gevz().gb_fgevat());

    yrg zhg bssfrg = 4_vfvmr;
    sbe v va 1.. {
        yrg zhg arkgtra = "  ".gb_fgevat();
        sbe j va fgngr.nf_olgrf().jvaqbjf(5) {
            vs cynagrq.pbagnvaf(j) {
                arkgtra.chfu('#');
            } ryfr {
                arkgtra.chfu(' ');
            }
        }
        yrg zhg juvgr = 0_vfvmr;
        sbe p va arkgtra.punef() {
            vs p.vf_juvgrfcnpr() {
                juvgr += 1;
            } ryfr {
                oernx;
            }
        }
        bssfrg -= juvgr - 4;
        arkgtra = "    ".gb_fgevat() + arkgtra.gevz() + &"    ".gb_fgevat();
        fgngr = arkgtra;

        vs !cnggreaf.vafreg(fgngr.pybar().gevz().gb_fgevat()) {
            yrg nafjre = fgngr.nf_olgrf().vgre().rahzrengr().sbyq(0, |npp, v| {
                vs *v.1 == 0k23 {
                    npp + v.0 nf vfvmr - bssfrg
                } ryfr {
                    npp
                }
            }) + (50_000_000_000 - v)
                * fgngr
                    .nf_olgrf()
                    .vgre()
                    .sbyq(0, |npp, v| vs *v == 0k23 { npp + 1 } ryfr { npp });
            cevagya!("NAFJRE : {}", nafjre);
            oernx;
        }
    }
}
