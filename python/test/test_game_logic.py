import unittest
import math
import sys
import os

# 実行中のスクリプトのディレクトリをパスに追加
sys.path.append(os.path.join(os.path.dirname(__file__), 'public'))

from game_logic import GameState

class TestGameLogic(unittest.TestCase):
    def setUp(self):
        self.total_laps = 3
        self.state = GameState(self.total_laps)

    def test_initial_state(self):
        self.assertEqual(self.state.get_x(), 10.0)
        self.assertEqual(self.state.get_y(), 1.0)
        self.assertEqual(self.state.get_z(), 0.0)
        self.assertEqual(self.state.get_lap(), 0)
        self.assertFalse(self.state.is_completed())

    def test_start_race(self):
        now = 1000.0
        self.state.start_race(now)
        self.assertTrue(self.state.race_started)
        self.assertEqual(self.state.start_time, now)

    def test_movement_forward(self):
        self.state.start_race(0.0)
        # 前進キー (ArrowUp)
        self.state.update(0.1, 100.0, ["ArrowUp"])
        
        # 速度がベーススピード（15.0）になっているか
        self.assertEqual(self.state.get_speed(), 15.0)
        
        # 座標が更新されているか（初期位置 x=10, z=0, rotation_y=0）
        # x += sin(0) * (0.1 * 15) = 10 + 0 = 10
        # z += cos(0) * (0.1 * 15) = 0 + 1.5 = 1.5
        self.assertAlmostEqual(self.state.get_z(), 1.5)
        self.assertEqual(self.state.get_x(), 10.0)

    def test_turn_and_move(self):
        self.state.start_race(0.0)
        # 左旋回 (ArrowLeft) - delta=0.1, rotation_speed=3.0 -> +0.3 rad
        self.state.update(0.1, 100.0, ["ArrowLeft", "ArrowUp"])
        
        self.assertAlmostEqual(self.state.get_rotation_y(), 0.3)
        
        # 次のフレームで移動
        # distance = 0.1 * 15.0 = 1.5
        # dx = sin(0.3) * 1.5
        # dz = cos(0.3) * 1.5
        expected_x = 10.0 + math.sin(0.3) * 1.5
        expected_z = 0.0 + math.cos(0.3) * 1.5
        
        self.state.update(0.1, 200.0, ["ArrowUp"])
        self.assertAlmostEqual(self.state.get_x(), expected_x)
        self.assertAlmostEqual(self.state.get_z(), expected_z)

    def test_lap_counting(self):
        self.state.start_race(0.0)
        # 円周を回るように角度を操作
        # 1周分（2π）以上の角度変化を与える
        self.state.total_angle = math.pi * 2.1
        self.state.update(0.1, 100.0, [])
        self.assertEqual(self.state.get_lap(), 1)
        
        self.state.total_angle = math.pi * 6.1
        self.state.update(0.1, 200.0, [])
        self.assertEqual(self.state.get_lap(), 3)
        self.assertTrue(self.state.is_completed())

if __name__ == '__main__':
    unittest.main()
