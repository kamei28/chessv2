#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "sleep.h"
#define LEVEL_MIN    5          // 最小レベル(桁数)
#define LEVEL_MAX   20          // 最大レベル(桁数)
void x_check(char *, char *);   // 正誤チェック

int main() {
    char no[LEVEL_MAX + 1];         // 正解文字列
    char x[LEVEL_MAX * 2];          // ユーザ入力
    int level;                      // レベル(桁数)
    int cnt = 0;                    // 回答数
    int i;
    time_t start_time, end_time;    // 測定時間
    double res_time;

    /** レベル(桁数)の入力 */
    srand(time(NULL));              // 乱数の種を設定
    printf("数値記憶トレーニング\n");
    do {
        printf("挑戦するレベル(%d~%d): ", LEVEL_MIN, LEVEL_MAX);
        scanf("%d", &level);
    } while(level < LEVEL_MIN || level > LEVEL_MAX);

    /** 問題生成 */
    no[0] = '1' + rand()%9;         // '1'~'9'の乱数を生成
    x[0] = '*';
    for (i = 1; i < level; i++) {
        no[i] = '0' + rand()%10;    // '0'~'9'の乱数を生成
        x[i] = '*';
    }
    no[level] = '\0';               // 最後に0
    x[level] = '*';

    /** ユーザの解答 */
    start_time = time(NULL);
    while (1) {
        printf("\n%d回目\n", ++cnt);    // 回答数を表示
        sleep(500);
        
        printf("Q: %s", no);            // 問題を表示
        fflush(stdout);                 // バッファを掃き出す
        sleep(125 * level);             // (125 * level)msの休止
        printf("\r%*s\r", level, "");   // 問題表示をクリア

        x_check(no, x);                 // ユーザ入力の正誤チェック(間違ってる箇所を*に変換する)
        printf("L: %s\n", x);           // 前回の入力を表示
        printf("A: ");
        fflush(stdout);

        scanf("%s", x);                // ユーザ入力
        if (strcmp(no, x) != 0) {       // strcmp関数で文字列を比較
            printf("間違いです。\n");
        } else {
            printf("正解です。\n");
            break;
        }
    }

    /** 結果出力 */
    end_time = time(NULL);
    res_time = difftime(end_time, start_time);  // 終了と開始時間の差を計算
    printf("%d回目で正解しました。\n", cnt);
    printf("%.1lf秒でした。\n", res_time);
    return 0;
}

/* 正誤チェック */
void x_check(char *no, char *x) {
    while (*no != '\0') {
        *x = *(no++) == *x? *x: '*';    // 値が違うなら'*'を代入
        *(x++);                         // ポインタを進める
    }
    *x = '\0';                          // 最後に0
}
