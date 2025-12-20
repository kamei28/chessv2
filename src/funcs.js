const { invoke } = window.__TAURI__.core;
let lastvalidmoves = [];
let nextturn = "w";

/** document.getElementByIdの省略 */
const getid = (id) => document.getElementById(id);

/** ボードの初期化を反映 */
window.addEventListener("DOMContentLoaded", () => 
  invoke("reset")
);

/** 駒の移動可能範囲を表示 */
const getValidMoves = async (piece, loc) => {

  // 移動可能範囲を取得
  lastvalidmoves = await invoke("get_valid_moves", { loc: Number(loc) });
  // lastvalidmoves = await invoke("test", { loc: Number(loc) });

  // getid("ns").innerText = lastvalidmoves;
  
  // return 0;

  // 移動可能範囲を表示
  for (v of lastvalidmoves) {
    let sp = document.createElement("span2");
    sp.classList.add(nextturn == piece.substring(0, 1)? "valid": "vaild2");
    getid(v.toString()).appendChild(sp);
  }
};

/** 前回の移動可能範囲を削除 */
const removeValidMoves = async () => {
  for (v of lastvalidmoves) {
    await getid(v.toString()).querySelector("span2").remove();
  }
  lastvalidmoves = [];
};

/** 可能であれば駒を動かす */
const movePiece = (fromp, to) => {
  if (lastvalidmoves.indexOf(Number(to.id)) != -1 && fromp.id.substring(0, 1) == nextturn) {

    // ターン交代
    nextturn = nextturn == "w" ? "b" : "w";
    getid("wb").innerText = {"w": "白", "b": "黒"}[nextturn];

    // ポーンの変身は画像とid変えてrustには変更後の駒情報渡せば多分おｋ

    // 駒の移動をサーバに反映
    console.log(`${fromp.id} moved from ${fromp.parentElement.id} to ${Number(to.id)}`);
    invoke("move_piece", {
      from: Number(fromp.parentElement.id),
      to: Number(to.id),
    }).then(loc => {
      
      // アンパッサンされた駒を削除
        console.log(loc);
      if (loc) {
        getid(loc).querySelector("img").remove();
      }
    });

    // 駒を移動させる
    try {
      to.querySelector("img").remove();
    } catch {}
    to.appendChild(fromp);

  } else {
    console.log(`${fromp.id} is not moved`);
  }
};
