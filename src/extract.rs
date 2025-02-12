use opencv::prelude::*;
use opencv::videoio::VideoCapture;

/// 飛び飛びのフレームも想定して，与えられた列のフレームを返す．
/// !! 結果はフレーム番号の昇順になる．
/// * 途中でフレームが空になっても許容することにするが，そこは飛ばす
/// * returns: [opencv::error::Result]<[Vec]<[Mat]>>
///   [VideoCaptureTrait::read](opencv::prelude::VideoCaptureTrait::read) が Err を返すときに同様にそれを返す
pub fn get_nth_frames(
    vc: &mut VideoCapture,
    ns: &[usize],
) -> opencv::error::Result<Vec<(usize, Mat)>> {
    let mut frames: Vec<usize> = ns.to_vec();
    frames.sort();
    let mut n = 0; // 今何フレーム目読んでるか
    let mut img = Mat::default();
    let mut result = vec![];
    for &next_target in &frames {
        // 次のところまで読み飛ばす
        while n < next_target {
            vc.grab()?;
            n += 1;
        }
        let read_st = vc.read(&mut img)?;
        n += 1;
        // 空じゃないフレームを読めてたら結果に追加
        if read_st {
            result.push((next_target, img.clone()));
        }
    }
    Ok(result)
}
