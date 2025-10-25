-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema mydb
-- -----------------------------------------------------
-- -----------------------------------------------------
-- Schema mana_vault
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Schema mana_vault
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `mana_vault` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci ;
USE `mana_vault` ;

-- -----------------------------------------------------
-- Table `mana_vault`.`card`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`card` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`card` (
  `general_id` VARCHAR(36) NOT NULL,
  `name` TEXT NOT NULL,
  `text` TEXT NULL DEFAULT NULL,
  PRIMARY KEY (`general_id`),
  UNIQUE INDEX `id_UNIQUE` (`general_id` ASC) VISIBLE)
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_0900_ai_ci;


-- -----------------------------------------------------
-- Table `mana_vault`.`card_printing`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`card_printing` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`card_printing` (
  `id` VARCHAR(36) NOT NULL,
  `card_id` VARCHAR(36) NOT NULL,
  `image_url` TEXT NULL DEFAULT NULL,
  `rarity` TEXT NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `idnew_table_UNIQUE` (`id` ASC) VISIBLE,
  INDEX `printing_to_card_idx` (`card_id` ASC) VISIBLE,
  CONSTRAINT `cp_printing_to_card`
    FOREIGN KEY (`card_id`)
    REFERENCES `mana_vault`.`card` (`general_id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_0900_ai_ci;


-- -----------------------------------------------------
-- Table `mana_vault`.`subtypes`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`subtypes` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`subtypes` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(100) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
  UNIQUE INDEX `name_UNIQUE` (`name` ASC) VISIBLE)
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_0900_ai_ci;


-- -----------------------------------------------------
-- Table `mana_vault`.`card_subtypes`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`card_subtypes` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`card_subtypes` (
  `card_id` VARCHAR(36) NOT NULL,
  `subtype_id` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`card_id`, `subtype_id`),
  INDEX `subtypetosubtype_idx` (`subtype_id` ASC) VISIBLE,
  CONSTRAINT `cs_subtype_to_card`
    FOREIGN KEY (`card_id`)
    REFERENCES `mana_vault`.`card` (`general_id`),
  CONSTRAINT `cs_subtype_to_subtype`
    FOREIGN KEY (`subtype_id`)
    REFERENCES `mana_vault`.`subtypes` (`id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_0900_ai_ci;


-- -----------------------------------------------------
-- Table `mana_vault`.`supertypes`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`supertypes` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`supertypes` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(100) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
  UNIQUE INDEX `name_UNIQUE` (`name` ASC) VISIBLE)
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_0900_ai_ci;


-- -----------------------------------------------------
-- Table `mana_vault`.`card_supertypes`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`card_supertypes` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`card_supertypes` (
  `card_id` VARCHAR(36) NOT NULL,
  `supertype_id` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`card_id`, `supertype_id`),
  INDEX `supertype_to_supertype_idx` (`supertype_id` ASC) VISIBLE,
  CONSTRAINT `cst_supertype_to_card`
    FOREIGN KEY (`card_id`)
    REFERENCES `mana_vault`.`card` (`general_id`),
  CONSTRAINT `cst_supertype_to_supertype`
    FOREIGN KEY (`supertype_id`)
    REFERENCES `mana_vault`.`supertypes` (`id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_0900_ai_ci;


-- -----------------------------------------------------
-- Table `mana_vault`.`types`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`types` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`types` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(100) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `idtypes_UNIQUE` (`id` ASC) VISIBLE,
  UNIQUE INDEX `name_UNIQUE` (`name` ASC) VISIBLE)
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_0900_ai_ci;


-- -----------------------------------------------------
-- Table `mana_vault`.`card_types`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`card_types` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`card_types` (
  `card_id` VARCHAR(36) NOT NULL,
  `type_id` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`card_id`, `type_id`),
  INDEX `type_to_type_idx` (`type_id` ASC) VISIBLE,
  CONSTRAINT `ct_type_to_card`
    FOREIGN KEY (`card_id`)
    REFERENCES `mana_vault`.`card` (`general_id`),
  CONSTRAINT `ct_type_to_type`
    FOREIGN KEY (`type_id`)
    REFERENCES `mana_vault`.`types` (`id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_0900_ai_ci;


-- -----------------------------------------------------
-- Table `mana_vault`.`creature_stats`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`creature_stats` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`creature_stats` (
  `card_id` VARCHAR(36) NOT NULL,
  `power` TEXT NULL DEFAULT NULL,
  `toughness` TEXT NULL DEFAULT NULL,
  PRIMARY KEY (`card_id`),
  CONSTRAINT `cstat_creature_to_card`
    FOREIGN KEY (`card_id`)
    REFERENCES `mana_vault`.`card` (`general_id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_0900_ai_ci;


-- -----------------------------------------------------
-- Table `mana_vault`.`keywords`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`keywords` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`keywords` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(100) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
  UNIQUE INDEX `name_UNIQUE` (`name` ASC) VISIBLE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`card_keywords`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`card_keywords` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`card_keywords` (
  `card_id` VARCHAR(36) NOT NULL,
  `keyword_id` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`card_id`, `keyword_id`),
  INDEX `keyword_to_keyword_idx` (`keyword_id` ASC) VISIBLE,
  CONSTRAINT `ck_keyword_to_card`
    FOREIGN KEY (`card_id`)
    REFERENCES `mana_vault`.`card` (`general_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `ck_keyword_to_keyword`
    FOREIGN KEY (`keyword_id`)
    REFERENCES `mana_vault`.`keywords` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`mana_cost`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`mana_cost` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`mana_cost` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `mana_pip` VARCHAR(10) NOT NULL,
  `icon_url` TEXT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
  UNIQUE INDEX `symbol_UNIQUE` (`mana_pip` ASC) VISIBLE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`card_mana_cost`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`card_mana_cost` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`card_mana_cost` (
  `card_id` VARCHAR(36) NOT NULL,
  `cost_id` INT UNSIGNED NOT NULL,
  `amount` INT UNSIGNED NULL,
  PRIMARY KEY (`card_id`, `cost_id`),
  INDEX `cost_to_cost_idx` (`cost_id` ASC) VISIBLE,
  CONSTRAINT `cmc_cost_to_card`
    FOREIGN KEY (`card_id`)
    REFERENCES `mana_vault`.`card` (`general_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `cmc_cost_to_cost`
    FOREIGN KEY (`cost_id`)
    REFERENCES `mana_vault`.`mana_cost` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`set`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`set` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`set` (
  `id` VARCHAR(36) NOT NULL,
  `name` VARCHAR(100) NOT NULL,
  `code` VARCHAR(3) NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`printing_in_set`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`printing_in_set` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`printing_in_set` (
  `printed_card_id` VARCHAR(36) NOT NULL,
  `set_id` VARCHAR(36) NOT NULL,
  `collector_number` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`printed_card_id`, `set_id`),
  INDEX `set_to_set_idx` (`set_id` ASC) VISIBLE,
  CONSTRAINT `pis_set_to_printing`
    FOREIGN KEY (`printed_card_id`)
    REFERENCES `mana_vault`.`card_printing` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `pis_set_to_set`
    FOREIGN KEY (`set_id`)
    REFERENCES `mana_vault`.`set` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`card_colors`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`card_colors` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`card_colors` (
  `card_id` VARCHAR(36) NOT NULL,
  `color` ENUM('W', 'U', 'B', 'R', 'G') NOT NULL,
  PRIMARY KEY (`card_id`, `color`),
  CONSTRAINT `card_to_card`
    FOREIGN KEY (`card_id`)
    REFERENCES `mana_vault`.`card` (`general_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`user`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`user` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`user` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `username` VARCHAR(50) NOT NULL,
  `password` VARCHAR(50) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `username_UNIQUE` (`username` ASC) VISIBLE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`user_cards`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`user_cards` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`user_cards` (
  `user_id` INT UNSIGNED NOT NULL,
  `printing_id` VARCHAR(36) NOT NULL,
  `amount` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`user_id`, `printing_id`),
  INDEX `printing_to_printing_idx` (`printing_id` ASC) VISIBLE,
  CONSTRAINT `uc_user_to_user`
    FOREIGN KEY (`user_id`)
    REFERENCES `mana_vault`.`user` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `uc_printing_to_printing`
    FOREIGN KEY (`printing_id`)
    REFERENCES `mana_vault`.`card_printing` (`card_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`format`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`format` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`format` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(100) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`deck`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`deck` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`deck` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `owner_id` INT UNSIGNED NOT NULL,
  `format_id` INT UNSIGNED NULL,
  `name` TEXT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
  INDEX `owner_to_owner_idx` (`owner_id` ASC) VISIBLE,
  INDEX `format_to_format_idx` (`format_id` ASC) VISIBLE,
  CONSTRAINT `d_owner_to_owner`
    FOREIGN KEY (`owner_id`)
    REFERENCES `mana_vault`.`user` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `d_format_to_format`
    FOREIGN KEY (`format_id`)
    REFERENCES `mana_vault`.`format` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`card_in_deck`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`card_in_deck` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`card_in_deck` (
  `printing_id` VARCHAR(36) NOT NULL,
  `deck_id` INT UNSIGNED NOT NULL,
  `amount` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`printing_id`, `deck_id`),
  INDEX `deck_to_deck_idx` (`deck_id` ASC) VISIBLE,
  CONSTRAINT `cid_deck_to_deck`
    FOREIGN KEY (`deck_id`)
    REFERENCES `mana_vault`.`deck` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `cid_card_to_card`
    FOREIGN KEY (`printing_id`)
    REFERENCES `mana_vault`.`card_printing` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`deck_commander`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`deck_commander` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`deck_commander` (
  `deck_id` INT UNSIGNED NOT NULL,
  `printing_id` VARCHAR(36) NOT NULL,
  PRIMARY KEY (`deck_id`, `printing_id`),
  INDEX `card_to_card_idx` (`printing_id` ASC) VISIBLE,
  CONSTRAINT `dc_deck_to_deck`
    FOREIGN KEY (`deck_id`)
    REFERENCES `mana_vault`.`deck` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `dc_card_to_card`
    FOREIGN KEY (`printing_id`)
    REFERENCES `mana_vault`.`card_printing` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mana_vault`.`card_legalities`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `mana_vault`.`card_legalities` ;

CREATE TABLE IF NOT EXISTS `mana_vault`.`card_legalities` (
  `card_id` VARCHAR(36) NOT NULL,
  `format_id` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`card_id`, `format_id`),
  INDEX `format_to_format_idx` (`format_id` ASC) VISIBLE,
  CONSTRAINT `cl_format_to_format`
    FOREIGN KEY (`format_id`)
    REFERENCES `mana_vault`.`format` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `cl_card_to_card`
    FOREIGN KEY (`card_id`)
    REFERENCES `mana_vault`.`card` (`general_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;

USE `mana_vault`;

DELIMITER $$

USE `mana_vault`$$
DROP TRIGGER IF EXISTS `mana_vault`.`user_cards_AFTER_UPDATE` $$
USE `mana_vault`$$
CREATE DEFINER = CURRENT_USER TRIGGER `mana_vault`.`user_cards_AFTER_UPDATE` AFTER UPDATE ON `user_cards` FOR EACH ROW
BEGIN
	IF NEW.amount = 0 THEN
		DELETE FROM user_cards uc
			WHERE NEW.printing_id = uc.printing_id
            AND NEW.user_id = uc.user_id;
	END IF;
END$$


USE `mana_vault`$$
DROP TRIGGER IF EXISTS `mana_vault`.`card_in_deck_AFTER_UPDATE` $$
USE `mana_vault`$$
CREATE DEFINER = CURRENT_USER TRIGGER `mana_vault`.`card_in_deck_AFTER_UPDATE` AFTER UPDATE ON `card_in_deck` FOR EACH ROW
BEGIN
	IF NEW.amount = 0 THEN
		DELETE FROM card_in_deck cid
			WHERE NEW.printing_id = cid.printing_id
            AND NEW.deck_id = cid.deck_id;
	END IF;
END$$


DELIMITER ;

SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
