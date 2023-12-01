package ui

import entity.board.Board
import entity.player.Player

interface UILib {
    fun drawBoard(board: Board)
    fun drawPlayer(player: Player)
}