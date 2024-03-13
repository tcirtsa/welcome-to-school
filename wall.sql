/*
 Navicat Premium Data Transfer

 Source Server         : docker
 Source Server Type    : MySQL
 Source Server Version : 80034
 Source Host           : localhost:3306
 Source Schema         : wall

 Target Server Type    : MySQL
 Target Server Version : 80034
 File Encoding         : 65001

 Date: 13/03/2024 14:23:52
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for db_account
-- ----------------------------
DROP TABLE IF EXISTS `db_account`;
CREATE TABLE `db_account`  (
  `id` int NOT NULL AUTO_INCREMENT,
  `username` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `email` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `role` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `avatar` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL,
  `register_time` datetime NULL DEFAULT NULL,
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `unique_email`(`email` ASC) USING BTREE,
  UNIQUE INDEX `unique_username`(`username` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 6 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of db_account
-- ----------------------------
INSERT INTO `db_account` VALUES (4, 'test', '3100443756@qq.com', '$2a$10$hW9tE2SzsfjV8Y82cG12YeEX/Wx6FUTMRuldFJqd5VME7qfTeegxq', 'user', '/avatar/eef59b02900d45529b3d72b192eb17dc', '2023-10-19 16:58:58');
INSERT INTO `db_account` VALUES (5, 'baitang', '1937362126@qq.cm', '$2a$10$obxJOho4G683.QCT2fUMzOF3zSNNc15/suckEAdiGfipZ0rPk226a', 'user', '/avatar/eef59b02900d45529b3d72b192eb17dc', '2023-11-07 17:33:57');
INSERT INTO `db_account` VALUES (6, 'heidan', '1937222126@qq.cm', '$2a$10$obxJOho4G683.QCT2fUMzOF3zSNNc15/suckEAdiGfipZ0rPk226a', 'admin', '/avatar/eef59b02900d45529b3d72b192eb17dc', '2023-11-07 17:33:57');

-- ----------------------------
-- Table structure for db_account_details
-- ----------------------------
DROP TABLE IF EXISTS `db_account_details`;
CREATE TABLE `db_account_details`  (
  `id` int NOT NULL AUTO_INCREMENT,
  `gender` tinyint NULL DEFAULT NULL,
  `phone` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NULL DEFAULT NULL,
  `qq` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NULL DEFAULT NULL,
  `wx` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NULL DEFAULT NULL,
  `desc` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NULL DEFAULT NULL,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 6 CHARACTER SET = utf8mb3 COLLATE = utf8mb3_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of db_account_details
-- ----------------------------
INSERT INTO `db_account_details` VALUES (4, 0, '12345678910', '1611545413', '116516515213', '哈守护佛奥红富士哦防化服hi神佛i啊回复饭还是');
INSERT INTO `db_account_details` VALUES (5, 0, '124897249', '1244441', '222', '我是魔王d+');
INSERT INTO `db_account_details` VALUES (6, 0, '1878454', '23352552', '5355dd', '黑蛋');




-- ----------------------------
-- Table structure for db_image_store
-- ----------------------------
DROP TABLE IF EXISTS `db_image_store`;
CREATE TABLE `db_image_store`  (
  `uid` int NULL DEFAULT NULL,
  `name` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NULL DEFAULT NULL,
  `time` datetime NULL DEFAULT NULL
) ENGINE = InnoDB CHARACTER SET = utf8mb3 COLLATE = utf8mb3_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of db_image_store
-- ----------------------------
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/109bd4fb2dbf49d0af5abd7047e02b21', '2023-10-30 21:01:48');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/0a3e53f0ca904587bf27411b5f03512d', '2023-10-30 21:04:17');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/7ced7054d6184328a7bfb8d3ccf1bd33', '2023-10-30 21:04:21');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/b0a73c1844f3495594adc3a66c44539a', '2023-10-30 21:04:24');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/56020e5fa201425ea0993e8c55fb53f7', '2023-10-30 21:04:36');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/1fbe86a08211416eb5b066d83cc18497', '2023-10-30 21:06:15');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/67696c5fdf294c4ba5b7e107bb29f7e6', '2023-10-30 21:06:21');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/44ed4c9a98364738aeead06d4cab4a76', '2023-10-30 21:06:42');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/866d17879d9d4924ba54513c76b2431d', '2023-10-30 21:07:21');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/129aa052a8c34a90a726dfaf55abd6ab', '2023-10-30 21:07:49');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/34c4b126c1ae4b8c89fcfb131ecf09d7', '2023-10-30 21:07:51');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/1f401b93ff664443b3ee9e6b19a3d60c', '2023-10-30 21:07:55');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/38d6183ed23e4cd593ac18a0f6710480', '2023-10-30 21:07:57');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/95a04b794b264e71b4419b3921799324', '2023-10-30 21:07:59');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/fafb1bdc952d4b3ab6ba314afa994aad', '2023-10-30 21:08:04');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/e5d9f6dac1194446a6d9a96f46836047', '2023-10-30 21:08:06');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/680b7e04c453457e92d046c53019ada0', '2023-10-30 21:08:34');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/70962f06a7e0416d956029628bbab140', '2023-10-30 21:08:41');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/34b50c974934410195e2cee3d0c3d79c', '2023-10-30 21:08:55');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/539c389173214326b5c0b507918ef74a', '2023-10-30 21:10:45');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/53da3e21049c4ffaac4b340eb40d45e3', '2023-10-30 21:11:29');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/97b260e9a07e489eb27ba29eb873ca43', '2023-10-30 21:11:34');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/cb782f8e520c4047a49f317a6cc7fe5b', '2023-10-30 21:11:43');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/f5d373e3a39d4607ba4475986fd49fd1', '2023-10-30 21:14:39');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/12a8e7ae3054453ea9c548eeda7f9b65', '2023-10-30 21:15:07');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/c3905b83eea64f99bbfa4b0aa109773b', '2023-10-30 21:15:19');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/9ec0e9f5777649eabc9221736a36b9f1', '2023-10-30 21:15:19');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/88084e32a14f46b6b7b323f34644f799', '2023-10-30 21:15:19');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/99aeaa2b9cde40c8a22c4821b554f942', '2023-10-30 21:15:20');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/d059a23d6f3e47d2b34fd78f5d7b072c', '2023-10-30 21:15:20');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/c462fa23189640d1aeb8ec8b97f66a50', '2023-10-30 21:15:20');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/6ddbe8f81edb4e77b4f90aadd5cfffe4', '2023-10-30 21:15:20');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/f1276af5eac34664801109c779286d17', '2023-10-30 21:15:20');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/f069a1aa8ebe441e87ce4ae89faf8dba', '2023-10-30 21:15:20');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/20b146ffc418457cadc206d175a062ee', '2023-10-30 21:15:21');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/378368d914a94a8da7e03c951c3be5e6', '2023-10-30 21:15:21');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/d2652bd4bd5149a5bd302a31ca022012', '2023-10-30 21:15:21');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/2346f872fa134ee4926180ff11d27fe2', '2023-10-30 21:15:21');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/152d22f9907041e69aebd8e2f32d7471', '2023-10-30 21:16:34');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/77026c5ed4e149bd80d45383fe4a512f', '2023-10-30 21:18:47');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/8d0a206582154b1bad0a670b8403e291', '2023-10-30 21:26:43');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/7fd75e788f954130a15863c192c573ef', '2023-10-30 21:27:02');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/624600ab58254b2288bce1fd9281b917', '2023-10-30 21:40:07');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231030/8103b844bb7a46129dd271baf0ff4d31', '2023-10-30 21:41:13');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231031/ec3ef355079740ad8ea635dca1a4e719', '2023-10-31 18:56:38');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231031/65520b63fe884cd5aa738f5f844ad472', '2023-10-31 18:59:39');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231031/d32543ac54af4be0b6522bd2a9f21902', '2023-10-31 19:14:22');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231031/0d6b9e3f180b4f0694ea0f4fda25baa7', '2023-10-31 19:21:15');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231031/7619087a4e2e40509dd4378975f71b73', '2023-10-31 19:21:55');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231031/e58964fa03064683847bf33125e3bbc0', '2023-10-31 19:22:20');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231031/3b62632cc3fb48639e7ccb05d8b5133b', '2023-10-31 19:33:07');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231031/1bddbee09be849fe858cbb72958d2a2d', '2023-10-31 19:33:19');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231102/f8392256fa35493f8da47a31c9e06c91', '2023-11-02 00:13:27');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231102/2dc510719c744b8e91d00acd3380613d', '2023-11-02 00:22:08');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231102/8c4904b934b24bed869a464bcbafa506', '2023-11-02 00:22:08');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231102/863bafa2697843afb95ff57e4d57c77b', '2023-11-02 00:23:28');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231103/006b9d66801948c2a0026d049cfee8f4', '2023-11-03 23:25:25');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231103/034d7e6c4373495f810e98480f7e8510', '2023-11-03 23:25:25');
INSERT INTO `db_image_store` VALUES (4, '/cache/20231103/60f625749d1240e78cffa44d4710b5f8', '2023-11-03 23:46:04');




SET FOREIGN_KEY_CHECKS = 1;
