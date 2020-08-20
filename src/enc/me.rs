use super::pinter::*;
use super::sad::*;
use super::*;
use crate::def::*;

const MAX_FIRST_SEARCH_STEP: i16 = 3;
const MAX_REFINE_SEARCH_STEP: i16 = 2;
const RASTER_SEARCH_STEP: i16 = 5;
const RASTER_SEARCH_THD: i16 = 5;
const REFINE_SEARCH_THD: i16 = 0;
const BI_STEP: i16 = 5;

impl EvcePInter {
    pub(crate) fn pinter_me_epzs(
        &mut self,
        x: i16,
        y: i16,
        log2_cuw: usize,
        log2_cuh: usize,
        refi: i8,
        lidx: usize,
        mvp: &[i16],
        mv: &mut [i16],
        bi: u8,
        refp: &Vec<Vec<EvcRefP>>,
    ) -> u32 {
        let mut mvc = [0i16; MV_D]; /* MV center for search */
        let mut gmvp = [0i16; MV_D]; /* MVP in frame cordinate */
        let mut range = [[0i16; MV_D]; MV_RANGE_DIM]; /* search range after clipping */
        let mut mvi = [0i16; MV_D];
        let mut mvt = [0i16; MV_D];
        let mut cost = std::u32::MAX;
        let mut cost_best = std::u32::MAX;
        let ri = refi; /* reference buffer index */
        let mut tmpstep = 0;
        let mut beststep = 0;

        gmvp[MV_X] = mvp[MV_X] + (x << 2);
        gmvp[MV_Y] = mvp[MV_Y] + (y << 2);

        if bi == BI_NORMAL {
            mvi[MV_X] = mv[MV_X] + (x << 2);
            mvi[MV_Y] = mv[MV_Y] + (y << 2);
            mvc[MV_X] = x + (mv[MV_X] >> 2);
            mvc[MV_Y] = y + (mv[MV_Y] >> 2);
        } else {
            mvi[MV_X] = mvp[MV_X] + (x << 2);
            mvi[MV_Y] = mvp[MV_Y] + (y << 2);
            mvc[MV_X] = x + (mvp[MV_X] >> 2);
            mvc[MV_Y] = y + (mvp[MV_Y] >> 2);
        }

        mvc[MV_X] = EVC_CLIP3(self.min_clip[MV_X], self.max_clip[MV_X], mvc[MV_X]);
        mvc[MV_Y] = EVC_CLIP3(self.min_clip[MV_Y], self.max_clip[MV_Y], mvc[MV_Y]);
        self.get_range_ipel(&mvc, &mut range, bi == BI_NORMAL, ri, lidx, refp);

        cost = self.me_ipel_diamond(
            x,
            y,
            1 << log2_cuw,
            1 << log2_cuh,
            ri,
            lidx,
            &mut range,
            &gmvp,
            &mvi,
            &mut mvt,
            bi,
            &mut tmpstep,
            MAX_FIRST_SEARCH_STEP,
            refp,
        );

        if cost < cost_best {
            cost_best = cost;
            mv[MV_X] = mvt[MV_X];
            mv[MV_Y] = mvt[MV_Y];
            if (mvp[MV_X] - mv[MV_X]).abs() < 2 && (mvp[MV_Y] - mv[MV_Y]).abs() < 2 {
                beststep = 0;
            } else {
                beststep = tmpstep;
            }
        }

        if bi == BI_NON && beststep > RASTER_SEARCH_THD {
            //TODO: cost = me_raster(pi, x, y, log2_cuw, log2_cuh, ri, lidx, range, gmvp, mvt);
            if cost < cost_best {
                beststep = RASTER_SEARCH_THD;

                cost_best = cost;

                mv[MV_X] = mvt[MV_X];
                mv[MV_Y] = mvt[MV_Y];
            }
        }

        while bi != BI_NORMAL && beststep > REFINE_SEARCH_THD {
            mvc[MV_X] = x + (mv[MV_X] >> 2);
            mvc[MV_Y] = y + (mv[MV_Y] >> 2);

            self.get_range_ipel(&mvc, &mut range, bi == BI_NORMAL, ri, lidx, refp);

            mvi[MV_X] = mv[MV_X] + (x << 2);
            mvi[MV_Y] = mv[MV_Y] + (y << 2);

            beststep = 0;
            cost = self.me_ipel_diamond(
                x,
                y,
                1 << log2_cuw,
                1 << log2_cuh,
                ri,
                lidx,
                &mut range,
                &gmvp,
                &mvi,
                &mut mvt,
                bi,
                &mut tmpstep,
                MAX_REFINE_SEARCH_STEP,
                refp,
            );
            if cost < cost_best {
                cost_best = cost;

                mv[MV_X] = mvt[MV_X];
                mv[MV_Y] = mvt[MV_Y];

                if (mvp[MV_X] - mv[MV_X]).abs() < 2 && (mvp[MV_Y] - mv[MV_Y]).abs() < 2 {
                    beststep = 0;
                } else {
                    beststep = tmpstep;
                }
            }
        }

        if self.me_level > ME_LEV_IPEL {
            /* sub-pel ME */
            //TODO:  cost = me_spel_pattern(pi, x, y, log2_cuw, log2_cuh, ri, lidx, gmvp, mv, mvt, bi);

            if cost < cost_best {
                cost_best = cost;

                mv[MV_X] = mvt[MV_X];
                mv[MV_Y] = mvt[MV_Y];
            }
        } else {
            mvc[MV_X] = x + (mv[MV_X] >> 2);
            mvc[MV_Y] = y + (mv[MV_Y] >> 2);

            self.get_range_ipel(&mvc, &mut range, bi == BI_NORMAL, ri, lidx, refp);

            mvi[MV_X] = mv[MV_X] + (x << 2);
            mvi[MV_Y] = mv[MV_Y] + (y << 2);
            //TODO:  cost = me_ipel_refinement(pi, x, y, log2_cuw, log2_cuh, ri, lidx, range, gmvp, mvi, mvt, bi, &tmpstep, MAX_REFINE_SEARCH_STEP);
            if cost < cost_best {
                cost_best = cost;

                mv[MV_X] = mvt[MV_X];
                mv[MV_Y] = mvt[MV_Y];
            }
        }

        cost_best
    }

    fn get_range_ipel(
        &mut self,
        mvc: &[i16],
        range: &mut [[i16; MV_D]],
        bi: bool,
        ri: i8,
        lidx: usize,
        refp: &Vec<Vec<EvcRefP>>,
    ) {
        let offset = self.gop_size >> 1;
        let max_search_range = EVC_CLIP3(
            self.max_search_range >> 2,
            self.max_search_range,
            (self.max_search_range * (self.poc - refp[ri as usize][lidx].poc as i32).abs() as i16
                + offset as i16)
                / self.gop_size as i16,
        );
        let search_range_x = if bi { BI_STEP } else { max_search_range };
        let search_range_y = if bi { BI_STEP } else { max_search_range };

        /* define search range for int-pel search and clip it if needs */
        range[MV_RANGE_MIN][MV_X] = EVC_CLIP3(
            self.min_clip[MV_X],
            self.max_clip[MV_X],
            mvc[MV_X] - search_range_x,
        );
        range[MV_RANGE_MAX][MV_X] = EVC_CLIP3(
            self.min_clip[MV_X],
            self.max_clip[MV_X],
            mvc[MV_X] + search_range_x,
        );
        range[MV_RANGE_MIN][MV_Y] = EVC_CLIP3(
            self.min_clip[MV_Y],
            self.max_clip[MV_Y],
            mvc[MV_Y] - search_range_y,
        );
        range[MV_RANGE_MAX][MV_Y] = EVC_CLIP3(
            self.min_clip[MV_Y],
            self.max_clip[MV_Y],
            mvc[MV_Y] + search_range_y,
        );

        assert!(range[MV_RANGE_MIN][MV_X] <= range[MV_RANGE_MAX][MV_X]);
        assert!(range[MV_RANGE_MIN][MV_Y] <= range[MV_RANGE_MAX][MV_Y]);
    }

    fn me_ipel_diamond(
        &mut self,
        x: i16,
        y: i16,
        cuw: usize,
        cuh: usize,
        refi: i8,
        lidx: usize,
        range: &mut [[i16; MV_D]],
        gmvp: &[i16],
        mvi: &[i16],
        mv: &mut [i16],
        bi: u8,
        beststep: &mut i16,
        faststep: i16,
        refp: &Vec<Vec<EvcRefP>>,
    ) -> u32 {
        let mut cost = std::u32::MAX;
        let mut cost_best = std::u32::MAX;
        let mut mv_bits = 0;
        let mut mv_x = 0;
        let mut mv_y = 0;
        let lidx_r = if lidx == REFP_0 { REFP_1 } else { REFP_0 };
        let mut mvc = [0i16; MV_D];
        let mut min_cmv_x = 0;
        let mut min_cmv_y = 0;
        let mut max_cmv_x = 0;
        let mut max_cmv_y = 0;

        let mut mvsize = 1;
        let mut not_found_best = 0;

        let mut best_mv_bits = 0;
        let mut step = 0;
        let mut mv_best_x = EVC_CLIP3(self.min_clip[MV_X], self.max_clip[MV_X], (mvi[MV_X] >> 2));
        let mut mv_best_y = EVC_CLIP3(self.min_clip[MV_Y], self.max_clip[MV_Y], (mvi[MV_Y] >> 2));

        let mut imv_x = mv_best_x;
        let mut imv_y = mv_best_y;

        loop {
            not_found_best += 1;

            if step <= 2 {
                min_cmv_x = if mv_best_x <= range[MV_RANGE_MIN][MV_X] {
                    mv_best_x
                } else {
                    mv_best_x - if bi == BI_NORMAL { BI_STEP } else { 2 }
                };
                min_cmv_y = if mv_best_y <= range[MV_RANGE_MIN][MV_Y] {
                    mv_best_y
                } else {
                    mv_best_y - if bi == BI_NORMAL { BI_STEP } else { 2 }
                };
                max_cmv_x = if mv_best_x >= range[MV_RANGE_MAX][MV_X] {
                    mv_best_x
                } else {
                    mv_best_x + if bi == BI_NORMAL { BI_STEP } else { 2 }
                };
                max_cmv_y = if mv_best_y >= range[MV_RANGE_MAX][MV_Y] {
                    mv_best_y
                } else {
                    mv_best_y + if bi == BI_NORMAL { BI_STEP } else { 2 }
                };
                mvsize = 1;

                for i in (min_cmv_y..=max_cmv_y).step_by(mvsize) {
                    for j in (min_cmv_x..=max_cmv_x).step_by(mvsize) {
                        mv_x = j;
                        mv_y = i;

                        if mv_x > range[MV_RANGE_MAX][MV_X]
                            || mv_x < range[MV_RANGE_MIN][MV_X]
                            || mv_y > range[MV_RANGE_MAX][MV_Y]
                            || mv_y < range[MV_RANGE_MIN][MV_Y]
                        {
                            cost = std::u32::MAX;
                        } else {
                            /* get MVD bits */
                            mv_bits = get_mv_bits(
                                (mv_x << 2) - gmvp[MV_X],
                                (mv_y << 2) - gmvp[MV_Y],
                                self.num_refp as usize,
                                refi as usize,
                            );

                            if bi != 0 {
                                mv_bits += self.mot_bits[lidx_r];
                            }

                            /* get MVD cost_best */
                            cost = MV_COST(self.lambda_mv, mv_bits);

                            if bi != 0 {
                                /* get sad */
                                if let Some(pic_r) = &refp[refi as usize][lidx].pic {
                                    let frame_r = &pic_r.borrow().frame;
                                    let plane_r = &frame_r.borrow().planes[Y_C];
                                    cost += evce_sad_bi_16b(
                                        x,
                                        y,
                                        mv_x,
                                        mv_y,
                                        cuw,
                                        cuh,
                                        &self.org_bi.data[Y_C],
                                        &plane_r.as_region(),
                                    ) >> 1;
                                }
                            } else {
                                /* get sad */
                                if let (Some(pic_o), Some(pic_r)) =
                                    (&self.pic_o, &refp[refi as usize][lidx].pic)
                                {
                                    let (frame_o, frame_r) =
                                        (&pic_o.borrow().frame, &pic_r.borrow().frame);
                                    let (plane_o, plane_r) = (
                                        &frame_o.borrow().planes[Y_C],
                                        &frame_r.borrow().planes[Y_C],
                                    );
                                    cost += evce_sad_16b(
                                        x,
                                        y,
                                        mv_x,
                                        mv_y,
                                        cuw,
                                        cuh,
                                        &plane_o.as_region(),
                                        &plane_r.as_region(),
                                    );
                                }
                            }

                            /* check if motion cost_best is less than minimum cost_best */
                            if cost < cost_best {
                                mv_best_x = mv_x;
                                mv_best_y = mv_y;
                                *beststep = 2;
                                not_found_best = 0;
                                cost_best = cost;
                                best_mv_bits = mv_bits;
                            }
                        }
                    }
                }

                mvc[MV_X] = mv_best_x;
                mvc[MV_Y] = mv_best_y;

                self.get_range_ipel(&mvc, range, bi == BI_NORMAL, refi, lidx, refp);

                step += 2;
            } else {
                let meidx = if step > 8 { 2 } else { 1 };
                let multi = step;

                for i in 0..16 {
                    if meidx == 1 && i > 8 {
                        continue;
                    }
                    if (step == 4) && (i == 1 || i == 3 || i == 5 || i == 7) {
                        continue;
                    }

                    mv_x =
                        imv_x + ((multi >> meidx) * tbl_diapos_partial[meidx - 1][i][MV_X] as i16);
                    mv_y =
                        imv_y + ((multi >> meidx) * tbl_diapos_partial[meidx - 1][i][MV_Y] as i16);

                    if mv_x > range[MV_RANGE_MAX][MV_X]
                        || mv_x < range[MV_RANGE_MIN][MV_X]
                        || mv_y > range[MV_RANGE_MAX][MV_Y]
                        || mv_y < range[MV_RANGE_MIN][MV_Y]
                    {
                        cost = std::u32::MAX;
                    } else {
                        /* get MVD bits */
                        mv_bits = get_mv_bits(
                            (mv_x << 2) - gmvp[MV_X],
                            (mv_y << 2) - gmvp[MV_Y],
                            self.num_refp as usize,
                            refi as usize,
                        );

                        if bi != 0 {
                            mv_bits += self.mot_bits[lidx_r];
                        }

                        /* get MVD cost_best */
                        cost = MV_COST(self.lambda_mv, mv_bits);

                        if bi != 0 {
                            /* get sad */
                            if let Some(pic_r) = &refp[refi as usize][lidx].pic {
                                let frame_r = &pic_r.borrow().frame;
                                let plane_r = &frame_r.borrow().planes[Y_C];
                                cost += evce_sad_bi_16b(
                                    x,
                                    y,
                                    mv_x,
                                    mv_y,
                                    cuw,
                                    cuh,
                                    &self.org_bi.data[Y_C],
                                    &plane_r.as_region(),
                                ) >> 1;
                            }
                        } else {
                            /* get sad */
                            if let (Some(pic_o), Some(pic_r)) =
                                (&self.pic_o, &refp[refi as usize][lidx].pic)
                            {
                                let (frame_o, frame_r) =
                                    (&pic_o.borrow().frame, &pic_r.borrow().frame);
                                let (plane_o, plane_r) =
                                    (&frame_o.borrow().planes[Y_C], &frame_r.borrow().planes[Y_C]);
                                cost += evce_sad_16b(
                                    x,
                                    y,
                                    mv_x,
                                    mv_y,
                                    cuw,
                                    cuh,
                                    &plane_o.as_region(),
                                    &plane_r.as_region(),
                                );
                            }
                        }

                        /* check if motion cost_best is less than minimum cost_best */
                        if cost < cost_best {
                            mv_best_x = mv_x;
                            mv_best_y = mv_y;
                            *beststep = step;
                            cost_best = cost;
                            best_mv_bits = mv_bits;
                            not_found_best = 0;
                        }
                    }
                }
            }

            if not_found_best == faststep {
                break;
            }

            if bi == BI_NORMAL {
                break;
            }

            step <<= 1;

            if step > self.max_search_range || (step << (-2)) > self.max_search_range {
                break;
            }
        }

        /* set best MV */
        mv[MV_X] = ((mv_best_x - x) << 2);
        mv[MV_Y] = ((mv_best_y - y) << 2);

        if bi != BI_NORMAL && best_mv_bits > 0 {
            self.mot_bits[lidx] = best_mv_bits;
        }

        cost_best
    }
}