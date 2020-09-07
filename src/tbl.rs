use super::api::*;
use super::def::*;
use super::util::*;

pub(crate) static tbl_cu_dim_offset: [usize; 3] = [0, MAX_CU_DIM, MAX_CU_DIM + (MAX_CU_DIM >> 2)];
pub(crate) static tbl_nb_siz_offset: [usize; 3] = [
    0,
    (MAX_CU_SIZE << 2) + 1,
    (MAX_CU_SIZE << 2) + 1 + (MAX_CU_SIZE << 1) + 1,
];

pub(crate) static evey_tbl_mpm: [[[u8; 5]; 6]; 6] = [
    [
        [0, 2, 3, 1, 4],
        [0, 2, 1, 3, 4],
        [0, 2, 1, 3, 4],
        [1, 2, 0, 3, 4],
        [0, 2, 1, 3, 4],
        [0, 1, 2, 3, 4],
    ],
    [
        [1, 0, 2, 3, 4],
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 4],
        [1, 2, 0, 3, 4],
        [0, 1, 3, 2, 4],
        [0, 2, 1, 4, 3],
    ],
    [
        [1, 0, 2, 3, 4],
        [1, 0, 2, 3, 4],
        [1, 0, 2, 3, 4],
        [2, 0, 1, 3, 4],
        [1, 0, 3, 2, 4],
        [0, 1, 2, 4, 3],
    ],
    [
        [1, 0, 2, 3, 4],
        [0, 2, 1, 3, 4],
        [1, 0, 2, 3, 4],
        [1, 2, 0, 3, 4],
        [0, 1, 2, 3, 4],
        [0, 2, 1, 4, 3],
    ],
    [
        [0, 1, 2, 3, 4],
        [0, 3, 2, 1, 4],
        [1, 0, 2, 3, 4],
        [1, 2, 0, 3, 4],
        [1, 2, 3, 0, 4],
        [0, 2, 1, 4, 3],
    ],
    [
        [0, 1, 2, 3, 4],
        [0, 1, 2, 4, 3],
        [0, 1, 2, 4, 3],
        [0, 2, 1, 4, 3],
        [0, 1, 2, 3, 4],
        [0, 1, 2, 4, 3],
    ],
];

pub(crate) static evc_tbl_log2: [u8; 257] = [
    /* 0, 1 */
    0, 0, /* 2, 3 */
    1, 1, /* 4 ~ 7 */
    2, 2, 2, 2, /* 8 ~ 15 */
    3, 3, 3, 3, 3, 3, 3, 3, /* 16 ~ 31 */
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, /* 31 ~ 63 */
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    /* 64 ~ 127 */
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    /* 128 ~ 255 */
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    /* 256 */
    8,
];

pub(crate) static evc_tbl_qp_chroma_ajudst_base: [i8; MAX_QP_TABLE_SIZE_EXT] = [
    -12, -11, -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 29, 29, 30, 31, 32, 32, 33,
    33, 34, 34, 35, 35, 36, 36, 36, 37, 37, 37, 38, 38, 39, 39, 40, 40, 40, 41, 41, 41,
];

pub(crate) const EVC_TBL_CHROMA_QP_OFFSET: i8 = 6 * (BIT_DEPTH as i8 - 8);

pub(crate) static evc_tbl_dq_scale_b: [i16; 6] = [40, 45, 51, 57, 64, 71];

#[rustfmt::skip]
pub(crate) static  evc_tbl_tm2:[[i8;2];2] = [
    [ 64, 64],
    [ 64,-64],
];

#[rustfmt::skip]
pub(crate) static  evc_tbl_tm4:[[i8;4];4] = [
    [ 64, 64, 64, 64],
    [ 84, 35,-35,-84],
    [ 64,-64,-64, 64],
    [ 35,-84, 84,-35]
];

#[rustfmt::skip]
pub(crate) static evc_tbl_tm8:[[i8;8];8] = [
    [ 64, 64, 64, 64, 64, 64, 64, 64],
    [ 89, 75, 50, 18,-18,-50,-75,-89],
    [ 84, 35,-35,-84,-84,-35, 35, 84],
    [ 75,-18,-89,-50, 50, 89, 18,-75],
    [ 64,-64,-64, 64, 64,-64,-64, 64],
    [ 50,-89, 18, 75,-75,-18, 89,-50],
    [ 35,-84, 84,-35,-35, 84,-84, 35],
    [ 18,-50, 75,-89, 89,-75, 50,-18]
];

#[rustfmt::skip]
pub(crate) static  evc_tbl_tm16:[[i8;16];16] = [
    [ 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64],
    [ 90, 87, 80, 70, 57, 43, 26,  9, -9,-26,-43,-57,-70,-80,-87,-90],
    [ 89, 75, 50, 18,-18,-50,-75,-89,-89,-75,-50,-18, 18, 50, 75, 89],
    [ 87, 57,  9,-43,-80,-90,-70,-26, 26, 70, 90, 80, 43, -9,-57,-87],
    [ 84, 35,-35,-84,-84,-35, 35, 84, 84, 35,-35,-84,-84,-35, 35, 84],
    [ 80,  9,-70,-87,-26, 57, 90, 43,-43,-90,-57, 26, 87, 70, -9,-80],
    [ 75,-18,-89,-50, 50, 89, 18,-75,-75, 18, 89, 50,-50,-89,-18, 75],
    [ 70,-43,-87,  9, 90, 26,-80,-57, 57, 80,-26,-90, -9, 87, 43,-70],
    [ 64,-64,-64, 64, 64,-64,-64, 64, 64,-64,-64, 64, 64,-64,-64, 64],
    [ 57,-80,-26, 90, -9,-87, 43, 70,-70,-43, 87,  9,-90, 26, 80,-57],
    [ 50,-89, 18, 75,-75,-18, 89,-50,-50, 89,-18,-75, 75, 18,-89, 50],
    [ 43,-90, 57, 26,-87, 70,  9,-80, 80, -9,-70, 87,-26,-57, 90,-43],
    [ 35,-84, 84,-35,-35, 84,-84, 35, 35,-84, 84,-35,-35, 84,-84, 35],
    [ 26,-70, 90,-80, 43,  9,-57, 87,-87, 57, -9,-43, 80,-90, 70,-26],
    [ 18,-50, 75,-89, 89,-75, 50,-18,-18, 50,-75, 89,-89, 75,-50, 18],
    [  9,-26, 43,-57, 70,-80, 87,-90, 90,-87, 80,-70, 57,-43, 26, -9]
];

#[rustfmt::skip]
pub(crate) static  evc_tbl_tm32:[[i8;32];32] = [
    [ 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64],
    [ 90, 90, 88, 85, 82, 78, 73, 67, 61, 54, 47, 39, 30, 22, 13,  4, -4,-13,-22,-30,-39,-47,-54,-61,-67,-73,-78,-82,-85,-88,-90,-90],
    [ 90, 87, 80, 70, 57, 43, 26,  9, -9,-26,-43,-57,-70,-80,-87,-90,-90,-87,-80,-70,-57,-43,-26, -9,  9, 26, 43, 57, 70, 80, 87, 90],
    [ 90, 82, 67, 47, 22, -4,-30,-54,-73,-85,-90,-88,-78,-61,-39,-13, 13, 39, 61, 78, 88, 90, 85, 73, 54, 30,  4,-22,-47,-67,-82,-90],
    [ 89, 75, 50, 18,-18,-50,-75,-89,-89,-75,-50,-18, 18, 50, 75, 89, 89, 75, 50, 18,-18,-50,-75,-89,-89,-75,-50,-18, 18, 50, 75, 89],
    [ 88, 67, 30,-13,-54,-82,-90,-78,-47, -4, 39, 73, 90, 85, 61, 22,-22,-61,-85,-90,-73,-39,  4, 47, 78, 90, 82, 54, 13,-30,-67,-88],
    [ 87, 57,  9,-43,-80,-90,-70,-26, 26, 70, 90, 80, 43, -9,-57,-87,-87,-57, -9, 43, 80, 90, 70, 26,-26,-70,-90,-80,-43,  9, 57, 87],
    [ 85, 47,-13,-67,-90,-73,-22, 39, 82, 88, 54, -4,-61,-90,-78,-30, 30, 78, 90, 61,  4,-54,-88,-82,-39, 22, 73, 90, 67, 13,-47,-85],
    [ 84, 35,-35,-84,-84,-35, 35, 84, 84, 35,-35,-84,-84,-35, 35, 84, 84, 35,-35,-84,-84,-35, 35, 84, 84, 35,-35,-84,-84,-35, 35, 84],
    [ 82, 22,-54,-90,-61, 13, 78, 85, 30,-47,-90,-67,  4, 73, 88, 39,-39,-88,-73, -4, 67, 90, 47,-30,-85,-78,-13, 61, 90, 54,-22,-82],
    [ 80,  9,-70,-87,-26, 57, 90, 43,-43,-90,-57, 26, 87, 70, -9,-80,-80, -9, 70, 87, 26,-57,-90,-43, 43, 90, 57,-26,-87,-70,  9, 80],
    [ 78, -4,-82,-73, 13, 85, 67,-22,-88,-61, 30, 90, 54,-39,-90,-47, 47, 90, 39,-54,-90,-30, 61, 88, 22,-67,-85,-13, 73, 82,  4,-78],
    [ 75,-18,-89,-50, 50, 89, 18,-75,-75, 18, 89, 50,-50,-89,-18, 75, 75,-18,-89,-50, 50, 89, 18,-75,-75, 18, 89, 50,-50,-89,-18, 75],
    [ 73,-30,-90,-22, 78, 67,-39,-90,-13, 82, 61,-47,-88, -4, 85, 54,-54,-85,  4, 88, 47,-61,-82, 13, 90, 39,-67,-78, 22, 90, 30,-73],
    [ 70,-43,-87,  9, 90, 26,-80,-57, 57, 80,-26,-90, -9, 87, 43,-70,-70, 43, 87, -9,-90,-26, 80, 57,-57,-80, 26, 90,  9,-87,-43, 70],
    [ 67,-54,-78, 39, 85,-22,-90,  4, 90, 13,-88,-30, 82, 47,-73,-61, 61, 73,-47,-82, 30, 88,-13,-90, -4, 90, 22,-85,-39, 78, 54,-67],
    [ 64,-64,-64, 64, 64,-64,-64, 64, 64,-64,-64, 64, 64,-64,-64, 64, 64,-64,-64, 64, 64,-64,-64, 64, 64,-64,-64, 64, 64,-64,-64, 64],
    [ 61,-73,-47, 82, 30,-88,-13, 90, -4,-90, 22, 85,-39,-78, 54, 67,-67,-54, 78, 39,-85,-22, 90,  4,-90, 13, 88,-30,-82, 47, 73,-61],
    [ 57,-80,-26, 90, -9,-87, 43, 70,-70,-43, 87,  9,-90, 26, 80,-57,-57, 80, 26,-90,  9, 87,-43,-70, 70, 43,-87, -9, 90,-26,-80, 57],
    [ 54,-85, -4, 88,-47,-61, 82, 13,-90, 39, 67,-78,-22, 90,-30,-73, 73, 30,-90, 22, 78,-67,-39, 90,-13,-82, 61, 47,-88,  4, 85,-54],
    [ 50,-89, 18, 75,-75,-18, 89,-50,-50, 89,-18,-75, 75, 18,-89, 50, 50,-89, 18, 75,-75,-18, 89,-50,-50, 89,-18,-75, 75, 18,-89, 50],
    [ 47,-90, 39, 54,-90, 30, 61,-88, 22, 67,-85, 13, 73,-82,  4, 78,-78, -4, 82,-73,-13, 85,-67,-22, 88,-61,-30, 90,-54,-39, 90,-47],
    [ 43,-90, 57, 26,-87, 70,  9,-80, 80, -9,-70, 87,-26,-57, 90,-43,-43, 90,-57,-26, 87,-70, -9, 80,-80,  9, 70,-87, 26, 57,-90, 43],
    [ 39,-88, 73, -4,-67, 90,-47,-30, 85,-78, 13, 61,-90, 54, 22,-82, 82,-22,-54, 90,-61,-13, 78,-85, 30, 47,-90, 67,  4,-73, 88,-39],
    [ 35,-84, 84,-35,-35, 84,-84, 35, 35,-84, 84,-35,-35, 84,-84, 35, 35,-84, 84,-35,-35, 84,-84, 35, 35,-84, 84,-35,-35, 84,-84, 35],
    [ 30,-78, 90,-61,  4, 54,-88, 82,-39,-22, 73,-90, 67,-13,-47, 85,-85, 47, 13,-67, 90,-73, 22, 39,-82, 88,-54, -4, 61,-90, 78,-30],
    [ 26,-70, 90,-80, 43,  9,-57, 87,-87, 57, -9,-43, 80,-90, 70,-26,-26, 70,-90, 80,-43, -9, 57,-87, 87,-57,  9, 43,-80, 90,-70, 26],
    [ 22,-61, 85,-90, 73,-39, -4, 47,-78, 90,-82, 54,-13,-30, 67,-88, 88,-67, 30, 13,-54, 82,-90, 78,-47,  4, 39,-73, 90,-85, 61,-22],
    [ 18,-50, 75,-89, 89,-75, 50,-18,-18, 50,-75, 89,-89, 75,-50, 18, 18,-50, 75,-89, 89,-75, 50,-18,-18, 50,-75, 89,-89, 75,-50, 18],
    [ 13,-39, 61,-78, 88,-90, 85,-73, 54,-30,  4, 22,-47, 67,-82, 90,-90, 82,-67, 47,-22, -4, 30,-54, 73,-85, 90,-88, 78,-61, 39,-13],
    [  9,-26, 43,-57, 70,-80, 87,-90, 90,-87, 80,-70, 57,-43, 26, -9, -9, 26,-43, 57,-70, 80,-87, 90,-90, 87,-80, 70,-57, 43,-26,  9],
    [  4,-13, 22,-30, 39,-47, 54,-61, 67,-73, 78,-82, 85,-88, 90,-90, 90,-90, 88,-85, 82,-78, 73,-67, 61,-54, 47,-39, 30,-22, 13, -4]
];

#[rustfmt::skip]
pub(crate) static evc_tbl_tm64:[[i8;64];64] = [
    [ 64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64,  64, ],
    [ 90,  90,  90,  89,  88,  87,  86,  84,  83,  81,  79,  76,  74,  71,  69,  66,  62,  59,  56,  52,  48,  45,  41,  37,  33,  28,  24,  20,  15,  11,   7,   2,  -2,  -7, -11, -15, -20, -24, -28, -33, -37, -41, -45, -48, -52, -56, -59, -62, -66, -69, -71, -74, -76, -79, -81, -83, -84, -86, -87, -88, -89, -90, -90, -90, ],
    [ 90,  90,  88,  85,  82,  78,  73,  67,  61,  54,  47,  39,  30,  22,  13,   4,  -4, -13, -22, -30, -39, -47, -54, -61, -67, -73, -78, -82, -85, -88, -90, -90, -90, -90, -88, -85, -82, -78, -73, -67, -61, -54, -47, -39, -30, -22, -13,  -4,   4,  13,  22,  30,  39,  47,  54,  61,  67,  73,  78,  82,  85,  88,  90,  90, ],
    [ 90,  88,  84,  79,  71,  62,  52,  41,  28,  15,   2, -11, -24, -37, -48, -59, -69, -76, -83, -87, -90, -90, -89, -86, -81, -74, -66, -56, -45, -33, -20,  -7,   7,  20,  33,  45,  56,  66,  74,  81,  86,  89,  90,  90,  87,  83,  76,  69,  59,  48,  37,  24,  11,  -2, -15, -28, -41, -52, -62, -71, -79, -84, -88, -90, ],
    [ 90,  87,  80,  70,  57,  43,  26,   9,  -9, -26, -43, -57, -70, -80, -87, -90, -90, -87, -80, -70, -57, -43, -26,  -9,   9,  26,  43,  57,  70,  80,  87,  90,  90,  87,  80,  70,  57,  43,  26,   9,  -9, -26, -43, -57, -70, -80, -87, -90, -90, -87, -80, -70, -57, -43, -26,  -9,   9,  26,  43,  57,  70,  80,  87,  90, ],
    [ 90,  84,  74,  59,  41,  20,  -2, -24, -45, -62, -76, -86, -90, -89, -83, -71, -56, -37, -15,   7,  28,  48,  66,  79,  87,  90,  88,  81,  69,  52,  33,  11, -11, -33, -52, -69, -81, -88, -90, -87, -79, -66, -48, -28,  -7,  15,  37,  56,  71,  83,  89,  90,  86,  76,  62,  45,  24,   2, -20, -41, -59, -74, -84, -90, ],
    [ 90,  82,  67,  47,  22,  -4, -30, -54, -73, -85, -90, -88, -78, -61, -39, -13,  13,  39,  61,  78,  88,  90,  85,  73,  54,  30,   4, -22, -47, -67, -82, -90, -90, -82, -67, -47, -22,   4,  30,  54,  73,  85,  90,  88,  78,  61,  39,  13, -13, -39, -61, -78, -88, -90, -85, -73, -54, -30,  -4,  22,  47,  67,  82,  90, ],
    [ 89,  79,  59,  33,   2, -28, -56, -76, -88, -90, -81, -62, -37,  -7,  24,  52,  74,  87,  90,  83,  66,  41,  11, -20, -48, -71, -86, -90, -84, -69, -45, -15,  15,  45,  69,  84,  90,  86,  71,  48,  20, -11, -41, -66, -83, -90, -87, -74, -52, -24,   7,  37,  62,  81,  90,  88,  76,  56,  28,  -2, -33, -59, -79, -89, ],
    [ 89,  75,  50,  18, -18, -50, -75, -89, -89, -75, -50, -18,  18,  50,  75,  89,  89,  75,  50,  18, -18, -50, -75, -89, -89, -75, -50, -18,  18,  50,  75,  89,  89,  75,  50,  18, -18, -50, -75, -89, -89, -75, -50, -18,  18,  50,  75,  89,  89,  75,  50,  18, -18, -50, -75, -89, -89, -75, -50, -18,  18,  50,  75,  89, ],
    [ 88,  71,  41,   2, -37, -69, -87, -89, -74, -45,  -7,  33,  66,  86,  90,  76,  48,  11, -28, -62, -84, -90, -79, -52, -15,  24,  59,  83,  90,  81,  56,  20, -20, -56, -81, -90, -83, -59, -24,  15,  52,  79,  90,  84,  62,  28, -11, -48, -76, -90, -86, -66, -33,   7,  45,  74,  89,  87,  69,  37,  -2, -41, -71, -88, ],
    [ 88,  67,  30, -13, -54, -82, -90, -78, -47,  -4,  39,  73,  90,  85,  61,  22, -22, -61, -85, -90, -73, -39,   4,  47,  78,  90,  82,  54,  13, -30, -67, -88, -88, -67, -30,  13,  54,  82,  90,  78,  47,   4, -39, -73, -90, -85, -61, -22,  22,  61,  85,  90,  73,  39,  -4, -47, -78, -90, -82, -54, -13,  30,  67,  88, ],
    [ 87,  62,  20, -28, -69, -89, -84, -56, -11,  37,  74,  90,  81,  48,   2, -45, -79, -90, -76, -41,   7,  52,  83,  90,  71,  33, -15, -59, -86, -88, -66, -24,  24,  66,  88,  86,  59,  15, -33, -71, -90, -83, -52,  -7,  41,  76,  90,  79,  45,  -2, -48, -81, -90, -74, -37,  11,  56,  84,  89,  69,  28, -20, -62, -87, ],
    [ 87,  57,   9, -43, -80, -90, -70, -26,  26,  70,  90,  80,  43,  -9, -57, -87, -87, -57,  -9,  43,  80,  90,  70,  26, -26, -70, -90, -80, -43,   9,  57,  87,  87,  57,   9, -43, -80, -90, -70, -26,  26,  70,  90,  80,  43,  -9, -57, -87, -87, -57,  -9,  43,  80,  90,  70,  26, -26, -70, -90, -80, -43,   9,  57,  87, ],
    [ 86,  52,  -2, -56, -87, -84, -48,   7,  59,  88,  83,  45, -11, -62, -89, -81, -41,  15,  66,  90,  79,  37, -20, -69, -90, -76, -33,  24,  71,  90,  74,  28, -28, -74, -90, -71, -24,  33,  76,  90,  69,  20, -37, -79, -90, -66, -15,  41,  81,  89,  62,  11, -45, -83, -88, -59,  -7,  48,  84,  87,  56,   2, -52, -86, ],
    [ 85,  47, -13, -67, -90, -73, -22,  39,  82,  88,  54,  -4, -61, -90, -78, -30,  30,  78,  90,  61,   4, -54, -88, -82, -39,  22,  73,  90,  67,  13, -47, -85, -85, -47,  13,  67,  90,  73,  22, -39, -82, -88, -54,   4,  61,  90,  78,  30, -30, -78, -90, -61,  -4,  54,  88,  82,  39, -22, -73, -90, -67, -13,  47,  85, ],
    [ 84,  41, -24, -76, -89, -56,   7,  66,  90,  69,  11, -52, -88, -79, -28,  37,  83,  86,  45, -20, -74, -90, -59,   2,  62,  90,  71,  15, -48, -87, -81, -33,  33,  81,  87,  48, -15, -71, -90, -62,  -2,  59,  90,  74,  20, -45, -86, -83, -37,  28,  79,  88,  52, -11, -69, -90, -66,  -7,  56,  89,  76,  24, -41, -84, ],
    [ 84,  35, -35, -84, -84, -35,  35,  84,  84,  35, -35, -84, -84, -35,  35,  84,  84,  35, -35, -84, -84, -35,  35,  84,  84,  35, -35, -84, -84, -35,  35,  84,  84,  35, -35, -84, -84, -35,  35,  84,  84,  35, -35, -84, -84, -35,  35,  84,  84,  35, -35, -84, -84, -35,  35,  84,  84,  35, -35, -84, -84, -35,  35,  84, ],
    [ 83,  28, -45, -88, -74, -11,  59,  90,  62,  -7, -71, -89, -48,  24,  81,  84,  33, -41, -87, -76, -15,  56,  90,  66,  -2, -69, -90, -52,  20,  79,  86,  37, -37, -86, -79, -20,  52,  90,  69,   2, -66, -90, -56,  15,  76,  87,  41, -33, -84, -81, -24,  48,  89,  71,   7, -62, -90, -59,  11,  74,  88,  45, -28, -83, ],
    [ 82,  22, -54, -90, -61,  13,  78,  85,  30, -47, -90, -67,   4,  73,  88,  39, -39, -88, -73,  -4,  67,  90,  47, -30, -85, -78, -13,  61,  90,  54, -22, -82, -82, -22,  54,  90,  61, -13, -78, -85, -30,  47,  90,  67,  -4, -73, -88, -39,  39,  88,  73,   4, -67, -90, -47,  30,  85,  78,  13, -61, -90, -54,  22,  82, ],
    [ 81,  15, -62, -90, -45,  37,  88,  69,  -7, -76, -84, -24,  56,  90,  52, -28, -86, -74,  -2,  71,  87,  33, -48, -90, -59,  20,  83,  79,  11, -66, -89, -41,  41,  89,  66, -11, -79, -83, -20,  59,  90,  48, -33, -87, -71,   2,  74,  86,  28, -52, -90, -56,  24,  84,  76,   7, -69, -88, -37,  45,  90,  62, -15, -81, ],
    [ 80,   9, -70, -87, -26,  57,  90,  43, -43, -90, -57,  26,  87,  70,  -9, -80, -80,  -9,  70,  87,  26, -57, -90, -43,  43,  90,  57, -26, -87, -70,   9,  80,  80,   9, -70, -87, -26,  57,  90,  43, -43, -90, -57,  26,  87,  70,  -9, -80, -80,  -9,  70,  87,  26, -57, -90, -43,  43,  90,  57, -26, -87, -70,   9,  80, ],
    [ 79,   2, -76, -81,  -7,  74,  83,  11, -71, -84, -15,  69,  86,  20, -66, -87, -24,  62,  88,  28, -59, -89, -33,  56,  90,  37, -52, -90, -41,  48,  90,  45, -45, -90, -48,  41,  90,  52, -37, -90, -56,  33,  89,  59, -28, -88, -62,  24,  87,  66, -20, -86, -69,  15,  84,  71, -11, -83, -74,   7,  81,  76,  -2, -79, ],
    [ 78,  -4, -82, -73,  13,  85,  67, -22, -88, -61,  30,  90,  54, -39, -90, -47,  47,  90,  39, -54, -90, -30,  61,  88,  22, -67, -85, -13,  73,  82,   4, -78, -78,   4,  82,  73, -13, -85, -67,  22,  88,  61, -30, -90, -54,  39,  90,  47, -47, -90, -39,  54,  90,  30, -61, -88, -22,  67,  85,  13, -73, -82,  -4,  78, ],
    [ 76, -11, -86, -62,  33,  90,  45, -52, -89, -24,  69,  83,   2, -81, -71,  20,  88,  56, -41, -90, -37,  59,  87,  15, -74, -79,   7,  84,  66, -28, -90, -48,  48,  90,  28, -66, -84,  -7,  79,  74, -15, -87, -59,  37,  90,  41, -56, -88, -20,  71,  81,  -2, -83, -69,  24,  89,  52, -45, -90, -33,  62,  86,  11, -76, ],
    [ 75, -18, -89, -50,  50,  89,  18, -75, -75,  18,  89,  50, -50, -89, -18,  75,  75, -18, -89, -50,  50,  89,  18, -75, -75,  18,  89,  50, -50, -89, -18,  75,  75, -18, -89, -50,  50,  89,  18, -75, -75,  18,  89,  50, -50, -89, -18,  75,  75, -18, -89, -50,  50,  89,  18, -75, -75,  18,  89,  50, -50, -89, -18,  75, ],
    [ 74, -24, -90, -37,  66,  81, -11, -88, -48,  56,  86,   2, -84, -59,  45,  89,  15, -79, -69,  33,  90,  28, -71, -76,  20,  90,  41, -62, -83,   7,  87,  52, -52, -87,  -7,  83,  62, -41, -90, -20,  76,  71, -28, -90, -33,  69,  79, -15, -89, -45,  59,  84,  -2, -86, -56,  48,  88,  11, -81, -66,  37,  90,  24, -74, ],
    [ 73, -30, -90, -22,  78,  67, -39, -90, -13,  82,  61, -47, -88,  -4,  85,  54, -54, -85,   4,  88,  47, -61, -82,  13,  90,  39, -67, -78,  22,  90,  30, -73, -73,  30,  90,  22, -78, -67,  39,  90,  13, -82, -61,  47,  88,   4, -85, -54,  54,  85,  -4, -88, -47,  61,  82, -13, -90, -39,  67,  78, -22, -90, -30,  73, ],
    [ 71, -37, -89,  -7,  86,  48, -62, -79,  24,  90,  20, -81, -59,  52,  84, -11, -90, -33,  74,  69, -41, -88,  -2,  87,  45, -66, -76,  28,  90,  15, -83, -56,  56,  83, -15, -90, -28,  76,  66, -45, -87,   2,  88,  41, -69, -74,  33,  90,  11, -84, -52,  59,  81, -20, -90, -24,  79,  62, -48, -86,   7,  89,  37, -71, ],
    [ 70, -43, -87,   9,  90,  26, -80, -57,  57,  80, -26, -90,  -9,  87,  43, -70, -70,  43,  87,  -9, -90, -26,  80,  57, -57, -80,  26,  90,   9, -87, -43,  70,  70, -43, -87,   9,  90,  26, -80, -57,  57,  80, -26, -90,  -9,  87,  43, -70, -70,  43,  87,  -9, -90, -26,  80,  57, -57, -80,  26,  90,   9, -87, -43,  70, ],
    [ 69, -48, -83,  24,  90,   2, -89, -28,  81,  52, -66, -71,  45,  84, -20, -90,  -7,  88,  33, -79, -56,  62,  74, -41, -86,  15,  90,  11, -87, -37,  76,  59, -59, -76,  37,  87, -11, -90, -15,  86,  41, -74, -62,  56,  79, -33, -88,   7,  90,  20, -84, -45,  71,  66, -52, -81,  28,  89,  -2, -90, -24,  83,  48, -69, ],
    [ 67, -54, -78,  39,  85, -22, -90,   4,  90,  13, -88, -30,  82,  47, -73, -61,  61,  73, -47, -82,  30,  88, -13, -90,  -4,  90,  22, -85, -39,  78,  54, -67, -67,  54,  78, -39, -85,  22,  90,  -4, -90, -13,  88,  30, -82, -47,  73,  61, -61, -73,  47,  82, -30, -88,  13,  90,   4, -90, -22,  85,  39, -78, -54,  67, ],
    [ 66, -59, -71,  52,  76, -45, -81,  37,  84, -28, -87,  20,  89, -11, -90,   2,  90,   7, -90, -15,  88,  24, -86, -33,  83,  41, -79, -48,  74,  56, -69, -62,  62,  69, -56, -74,  48,  79, -41, -83,  33,  86, -24, -88,  15,  90,  -7, -90,  -2,  90,  11, -89, -20,  87,  28, -84, -37,  81,  45, -76, -52,  71,  59, -66, ],
    [ 64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64,  64, -64, -64,  64, ],
    [ 62, -69, -56,  74,  48, -79, -41,  83,  33, -86, -24,  88,  15, -90,  -7,  90,  -2, -90,  11,  89, -20, -87,  28,  84, -37, -81,  45,  76, -52, -71,  59,  66, -66, -59,  71,  52, -76, -45,  81,  37, -84, -28,  87,  20, -89, -11,  90,   2, -90,   7,  90, -15, -88,  24,  86, -33, -83,  41,  79, -48, -74,  56,  69, -62, ],
    [ 61, -73, -47,  82,  30, -88, -13,  90,  -4, -90,  22,  85, -39, -78,  54,  67, -67, -54,  78,  39, -85, -22,  90,   4, -90,  13,  88, -30, -82,  47,  73, -61, -61,  73,  47, -82, -30,  88,  13, -90,   4,  90, -22, -85,  39,  78, -54, -67,  67,  54, -78, -39,  85,  22, -90,  -4,  90, -13, -88,  30,  82, -47, -73,  61, ],
    [ 59, -76, -37,  87,  11, -90,  15,  86, -41, -74,  62,  56, -79, -33,  88,   7, -90,  20,  84, -45, -71,  66,  52, -81, -28,  89,   2, -90,  24,  83, -48, -69,  69,  48, -83, -24,  90,  -2, -89,  28,  81, -52, -66,  71,  45, -84, -20,  90,  -7, -88,  33,  79, -56, -62,  74,  41, -86, -15,  90, -11, -87,  37,  76, -59, ],
    [ 57, -80, -26,  90,  -9, -87,  43,  70, -70, -43,  87,   9, -90,  26,  80, -57, -57,  80,  26, -90,   9,  87, -43, -70,  70,  43, -87,  -9,  90, -26, -80,  57,  57, -80, -26,  90,  -9, -87,  43,  70, -70, -43,  87,   9, -90,  26,  80, -57, -57,  80,  26, -90,   9,  87, -43, -70,  70,  43, -87,  -9,  90, -26, -80,  57, ],
    [ 56, -83, -15,  90, -28, -76,  66,  45, -87,  -2,  88, -41, -69,  74,  33, -90,  11,  84, -52, -59,  81,  20, -90,  24,  79, -62, -48,  86,   7, -89,  37,  71, -71, -37,  89,  -7, -86,  48,  62, -79, -24,  90, -20, -81,  59,  52, -84, -11,  90, -33, -74,  69,  41, -88,   2,  87, -45, -66,  76,  28, -90,  15,  83, -56, ],
    [ 54, -85,  -4,  88, -47, -61,  82,  13, -90,  39,  67, -78, -22,  90, -30, -73,  73,  30, -90,  22,  78, -67, -39,  90, -13, -82,  61,  47, -88,   4,  85, -54, -54,  85,   4, -88,  47,  61, -82, -13,  90, -39, -67,  78,  22, -90,  30,  73, -73, -30,  90, -22, -78,  67,  39, -90,  13,  82, -61, -47,  88,  -4, -85,  54, ],
    [ 52, -87,   7,  83, -62, -41,  90, -20, -76,  71,  28, -90,  33,  69, -79, -15,  89, -45, -59,  84,   2, -86,  56,  48, -88,  11,  81, -66, -37,  90, -24, -74,  74,  24, -90,  37,  66, -81, -11,  88, -48, -56,  86,  -2, -84,  59,  45, -89,  15,  79, -69, -33,  90, -28, -71,  76,  20, -90,  41,  62, -83,  -7,  87, -52, ],
    [ 50, -89,  18,  75, -75, -18,  89, -50, -50,  89, -18, -75,  75,  18, -89,  50,  50, -89,  18,  75, -75, -18,  89, -50, -50,  89, -18, -75,  75,  18, -89,  50,  50, -89,  18,  75, -75, -18,  89, -50, -50,  89, -18, -75,  75,  18, -89,  50,  50, -89,  18,  75, -75, -18,  89, -50, -50,  89, -18, -75,  75,  18, -89,  50, ],
    [ 48, -90,  28,  66, -84,   7,  79, -74, -15,  87, -59, -37,  90, -41, -56,  88, -20, -71,  81,   2, -83,  69,  24, -89,  52,  45, -90,  33,  62, -86,  11,  76, -76, -11,  86, -62, -33,  90, -45, -52,  89, -24, -69,  83,  -2, -81,  71,  20, -88,  56,  41, -90,  37,  59, -87,  15,  74, -79,  -7,  84, -66, -28,  90, -48, ],
    [ 47, -90,  39,  54, -90,  30,  61, -88,  22,  67, -85,  13,  73, -82,   4,  78, -78,  -4,  82, -73, -13,  85, -67, -22,  88, -61, -30,  90, -54, -39,  90, -47, -47,  90, -39, -54,  90, -30, -61,  88, -22, -67,  85, -13, -73,  82,  -4, -78,  78,   4, -82,  73,  13, -85,  67,  22, -88,  61,  30, -90,  54,  39, -90,  47, ],
    [ 45, -90,  48,  41, -90,  52,  37, -90,  56,  33, -89,  59,  28, -88,  62,  24, -87,  66,  20, -86,  69,  15, -84,  71,  11, -83,  74,   7, -81,  76,   2, -79,  79,  -2, -76,  81,  -7, -74,  83, -11, -71,  84, -15, -69,  86, -20, -66,  87, -24, -62,  88, -28, -59,  89, -33, -56,  90, -37, -52,  90, -41, -48,  90, -45, ],
    [ 43, -90,  57,  26, -87,  70,   9, -80,  80,  -9, -70,  87, -26, -57,  90, -43, -43,  90, -57, -26,  87, -70,  -9,  80, -80,   9,  70, -87,  26,  57, -90,  43,  43, -90,  57,  26, -87,  70,   9, -80,  80,  -9, -70,  87, -26, -57,  90, -43, -43,  90, -57, -26,  87, -70,  -9,  80, -80,   9,  70, -87,  26,  57, -90,  43, ],
    [ 41, -89,  66,  11, -79,  83, -20, -59,  90, -48, -33,  87, -71,  -2,  74, -86,  28,  52, -90,  56,  24, -84,  76,  -7, -69,  88, -37, -45,  90, -62, -15,  81, -81,  15,  62, -90,  45,  37, -88,  69,   7, -76,  84, -24, -56,  90, -52, -28,  86, -74,   2,  71, -87,  33,  48, -90,  59,  20, -83,  79, -11, -66,  89, -41, ],
    [ 39, -88,  73,  -4, -67,  90, -47, -30,  85, -78,  13,  61, -90,  54,  22, -82,  82, -22, -54,  90, -61, -13,  78, -85,  30,  47, -90,  67,   4, -73,  88, -39, -39,  88, -73,   4,  67, -90,  47,  30, -85,  78, -13, -61,  90, -54, -22,  82, -82,  22,  54, -90,  61,  13, -78,  85, -30, -47,  90, -67,  -4,  73, -88,  39, ],
    [ 37, -86,  79, -20, -52,  90, -69,   2,  66, -90,  56,  15, -76,  87, -41, -33,  84, -81,  24,  48, -89,  71,  -7, -62,  90, -59, -11,  74, -88,  45,  28, -83,  83, -28, -45,  88, -74,  11,  59, -90,  62,   7, -71,  89, -48, -24,  81, -84,  33,  41, -87,  76, -15, -56,  90, -66,  -2,  69, -90,  52,  20, -79,  86, -37, ],
    [ 35, -84,  84, -35, -35,  84, -84,  35,  35, -84,  84, -35, -35,  84, -84,  35,  35, -84,  84, -35, -35,  84, -84,  35,  35, -84,  84, -35, -35,  84, -84,  35,  35, -84,  84, -35, -35,  84, -84,  35,  35, -84,  84, -35, -35,  84, -84,  35,  35, -84,  84, -35, -35,  84, -84,  35,  35, -84,  84, -35, -35,  84, -84,  35, ],
    [ 33, -81,  87, -48, -15,  71, -90,  62,  -2, -59,  90, -74,  20,  45, -86,  83, -37, -28,  79, -88,  52,  11, -69,  90, -66,   7,  56, -89,  76, -24, -41,  84, -84,  41,  24, -76,  89, -56,  -7,  66, -90,  69, -11, -52,  88, -79,  28,  37, -83,  86, -45, -20,  74, -90,  59,   2, -62,  90, -71,  15,  48, -87,  81, -33, ],
    [ 30, -78,  90, -61,   4,  54, -88,  82, -39, -22,  73, -90,  67, -13, -47,  85, -85,  47,  13, -67,  90, -73,  22,  39, -82,  88, -54,  -4,  61, -90,  78, -30, -30,  78, -90,  61,  -4, -54,  88, -82,  39,  22, -73,  90, -67,  13,  47, -85,  85, -47, -13,  67, -90,  73, -22, -39,  82, -88,  54,   4, -61,  90, -78,  30, ],
    [ 28, -74,  90, -71,  24,  33, -76,  90, -69,  20,  37, -79,  90, -66,  15,  41, -81,  89, -62,  11,  45, -83,  88, -59,   7,  48, -84,  87, -56,   2,  52, -86,  86, -52,  -2,  56, -87,  84, -48,  -7,  59, -88,  83, -45, -11,  62, -89,  81, -41, -15,  66, -90,  79, -37, -20,  69, -90,  76, -33, -24,  71, -90,  74, -28, ],
    [ 26, -70,  90, -80,  43,   9, -57,  87, -87,  57,  -9, -43,  80, -90,  70, -26, -26,  70, -90,  80, -43,  -9,  57, -87,  87, -57,   9,  43, -80,  90, -70,  26,  26, -70,  90, -80,  43,   9, -57,  87, -87,  57,  -9, -43,  80, -90,  70, -26, -26,  70, -90,  80, -43,  -9,  57, -87,  87, -57,   9,  43, -80,  90, -70,  26, ],
    [ 24, -66,  88, -86,  59, -15, -33,  71, -90,  83, -52,   7,  41, -76,  90, -79,  45,   2, -48,  81, -90,  74, -37, -11,  56, -84,  89, -69,  28,  20, -62,  87, -87,  62, -20, -28,  69, -89,  84, -56,  11,  37, -74,  90, -81,  48,  -2, -45,  79, -90,  76, -41,  -7,  52, -83,  90, -71,  33,  15, -59,  86, -88,  66, -24, ],
    [ 22, -61,  85, -90,  73, -39,  -4,  47, -78,  90, -82,  54, -13, -30,  67, -88,  88, -67,  30,  13, -54,  82, -90,  78, -47,   4,  39, -73,  90, -85,  61, -22, -22,  61, -85,  90, -73,  39,   4, -47,  78, -90,  82, -54,  13,  30, -67,  88, -88,  67, -30, -13,  54, -82,  90, -78,  47,  -4, -39,  73, -90,  85, -61,  22, ],
    [ 20, -56,  81, -90,  83, -59,  24,  15, -52,  79, -90,  84, -62,  28,  11, -48,  76, -90,  86, -66,  33,   7, -45,  74, -89,  87, -69,  37,   2, -41,  71, -88,  88, -71,  41,  -2, -37,  69, -87,  89, -74,  45,  -7, -33,  66, -86,  90, -76,  48, -11, -28,  62, -84,  90, -79,  52, -15, -24,  59, -83,  90, -81,  56, -20, ],
    [ 18, -50,  75, -89,  89, -75,  50, -18, -18,  50, -75,  89, -89,  75, -50,  18,  18, -50,  75, -89,  89, -75,  50, -18, -18,  50, -75,  89, -89,  75, -50,  18,  18, -50,  75, -89,  89, -75,  50, -18, -18,  50, -75,  89, -89,  75, -50,  18,  18, -50,  75, -89,  89, -75,  50, -18, -18,  50, -75,  89, -89,  75, -50,  18, ],
    [ 15, -45,  69, -84,  90, -86,  71, -48,  20,  11, -41,  66, -83,  90, -87,  74, -52,  24,   7, -37,  62, -81,  90, -88,  76, -56,  28,   2, -33,  59, -79,  89, -89,  79, -59,  33,  -2, -28,  56, -76,  88, -90,  81, -62,  37,  -7, -24,  52, -74,  87, -90,  83, -66,  41, -11, -20,  48, -71,  86, -90,  84, -69,  45, -15, ],
    [ 13, -39,  61, -78,  88, -90,  85, -73,  54, -30,   4,  22, -47,  67, -82,  90, -90,  82, -67,  47, -22,  -4,  30, -54,  73, -85,  90, -88,  78, -61,  39, -13, -13,  39, -61,  78, -88,  90, -85,  73, -54,  30,  -4, -22,  47, -67,  82, -90,  90, -82,  67, -47,  22,   4, -30,  54, -73,  85, -90,  88, -78,  61, -39,  13, ],
    [ 11, -33,  52, -69,  81, -88,  90, -87,  79, -66,  48, -28,   7,  15, -37,  56, -71,  83, -89,  90, -86,  76, -62,  45, -24,   2,  20, -41,  59, -74,  84, -90,  90, -84,  74, -59,  41, -20,  -2,  24, -45,  62, -76,  86, -90,  89, -83,  71, -56,  37, -15,  -7,  28, -48,  66, -79,  87, -90,  88, -81,  69, -52,  33, -11, ],
    [  9, -26,  43, -57,  70, -80,  87, -90,  90, -87,  80, -70,  57, -43,  26,  -9,  -9,  26, -43,  57, -70,  80, -87,  90, -90,  87, -80,  70, -57,  43, -26,   9,   9, -26,  43, -57,  70, -80,  87, -90,  90, -87,  80, -70,  57, -43,  26,  -9,  -9,  26, -43,  57, -70,  80, -87,  90, -90,  87, -80,  70, -57,  43, -26,   9, ],
    [  7, -20,  33, -45,  56, -66,  74, -81,  86, -89,  90, -90,  87, -83,  76, -69,  59, -48,  37, -24,  11,   2, -15,  28, -41,  52, -62,  71, -79,  84, -88,  90, -90,  88, -84,  79, -71,  62, -52,  41, -28,  15,  -2, -11,  24, -37,  48, -59,  69, -76,  83, -87,  90, -90,  89, -86,  81, -74,  66, -56,  45, -33,  20,  -7, ],
    [  4, -13,  22, -30,  39, -47,  54, -61,  67, -73,  78, -82,  85, -88,  90, -90,  90, -90,  88, -85,  82, -78,  73, -67,  61, -54,  47, -39,  30, -22,  13,  -4,  -4,  13, -22,  30, -39,  47, -54,  61, -67,  73, -78,  82, -85,  88, -90,  90, -90,  90, -88,  85, -82,  78, -73,  67, -61,  54, -47,  39, -30,  22, -13,   4, ],
    [  2,  -7,  11, -15,  20, -24,  28, -33,  37, -41,  45, -48,  52, -56,  59, -62,  66, -69,  71, -74,  76, -79,  81, -83,  84, -86,  87, -88,  89, -90,  90, -90,  90, -90,  90, -89,  88, -87,  86, -84,  83, -81,  79, -76,  74, -71,  69, -66,  62, -59,  56, -52,  48, -45,  41, -37,  33, -28,  24, -20,  15, -11,   7,  -2, ],
];

pub(crate) static evc_tbl_df_st: [[u8; 52]; 4] = [
    /* intra */
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2,
        2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 8, 9, 10, 11, 12, 12, 12, 12, 12,
    ],
    /* non-zero coefficient(s) for luma */
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,
        1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 7, 8, 9, 10, 11, 11, 11, 11, 11,
    ],
    /* no non-zero coefficient & mvd >= 4 */
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 10,
    ],
    /* no deblock */
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
];

/*
pub(crate) static evc_tbl_tm[i8;MAX_CU_DEPTH] =
{
    evc_tbl_tm2[0],
    evc_tbl_tm4[0],
    evc_tbl_tm8[0],
    evc_tbl_tm16[0],
    evc_tbl_tm32[0],
    evc_tbl_tm64[0],
};*/

lazy_static! {
    pub(crate) static ref evc_scan_tbl: [Box<[u16]>; MAX_CU_LOG2] = {
        [
            scan_tbl(2),
            scan_tbl(4),
            scan_tbl(8),
            scan_tbl(16),
            scan_tbl(32),
            scan_tbl(64),
        ]
    };
    pub(crate) static ref evc_tbl_tr: [Box<[i16]>; MAX_CU_LOG2] = {
        [
            evc_init_multi_tbl(2),
            evc_init_multi_tbl(4),
            evc_init_multi_tbl(8),
            evc_init_multi_tbl(16),
            evc_init_multi_tbl(32),
            evc_init_multi_tbl(64),
        ]
    };
    pub(crate) static ref evc_tbl_inv_tr: [Box<[i16]>; MAX_CU_LOG2] = {
        [
            evc_init_multi_inv_tbl(2),
            evc_init_multi_inv_tbl(4),
            evc_init_multi_inv_tbl(8),
            evc_init_multi_inv_tbl(16),
            evc_init_multi_inv_tbl(32),
            evc_init_multi_inv_tbl(64),
        ]
    };
}
