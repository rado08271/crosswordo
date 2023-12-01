package ui

import entity.board.Board
import entity.player.Player
import common.constant.Settings

class Console: UILib {

    override fun drawBoard(board: Board) {
        board.board.forEachIndexed { index, s ->
            if (index % Settings.COLS == 0) println()
            print("|\t${s}\t|")
        }
    }

    override fun drawPlayer(player: Player) {
        println("nothing yet")
    }
}