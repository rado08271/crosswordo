package common.constant

import kotlin.math.max

object Settings {
    var COLS: Int = 20; // this is a default value
    var ROWS: Int = 20; // this is a default value
    var MAX: Int = max(ROWS, COLS)
//    var MAX: Int = 4

    fun boardSize(rows: Int, cols: Int) {
        ROWS = rows
        COLS = cols
    }
}