use mana_vault;

-- -----------------------------------------------------
-- formats table data
-- -----------------------------------------------------
DELETE FROM format;
INSERT IGNORE INTO format (id, name) VALUES 
    (1, 'Standard'),
    (2, 'Pioneer'),
    (3, 'Modern'),
    (4, 'Legacy'),
    (5, 'Vintage'),
    (6, 'Commander'),
    (7, 'Oathbreaker'),
    (8, 'Alchemy'),
    (9, 'Historic'),
    (10, 'Brawl'),
    (11, 'Timeless'),
    (12, 'Pauper'),
    (13, 'Penny');

-- -----------------------------------------------------
-- mana_cost table data
-- -----------------------------------------------------
DELETE FROM mana_cost;
INSERT IGNORE INTO mana_cost (id, mana_pip) VALUES
-- Coloured Mana
    (1, '{W}'),
    (2, '{U}'),
    (3, '{B}'),
    (4, '{R}'),
    (5, '{G}'),
    (6, '{C}'),
-- Generic Mana
    (7, '{0}'),
    (8, '{1}'),
    (9, '{2}'),
    (10, '{3}'),
    (11, '{4}'),
    (12, '{5}'),
    (13, '{6}'),
    (14, '{7}'),
    (15, '{8}'),
    (16, '{9}'),
    (17, '{10}'),
    (18, '{11}'),
    (19, '{12}'),
    (20, '{13}'),
    (21, '{14}'),
    (22, '{15}'),
    (23, '{16}'),
    (24, '{17}'),
    (25, '{18}'),
    (26, '{19}'),
    (27, '{20}'),
    (28, '{X}'),
-- Simple Hybrid
    (29, '{W/U}'),
    (30, '{W/B}'),
    (31, '{B/R}'),
    (32, '{B/G}'),
    (33, '{U/B}'),
    (34, '{U/R}'),
    (35, '{R/G}'),
    (36, '{R/W}'),
    (37, '{G/W}'),
    (38, '{G/U}'),
-- Phyrexian Mana
    (39, '{H}'),
    (40, '{W/P}'),
    (41, '{U/P}'),
    (42, '{B/P}'),
    (43, '{R/P}'),
    (44, '{G/P}'),
    (45, '{C/P}'),
-- Hybrid Phyrexian Mana
    (46, '{W/U/P}'),
    (47, '{W/B/P}'),
    (48, '{B/R/P}'),
    (49, '{B/G/P}'),
    (50, '{U/B/P}'),
    (51, '{U/R/P}'),
    (52, '{R/G/P}'),
    (53, '{R/W/P}'),
    (54, '{G/W/P}'),
    (55, '{G/U/P}'),
-- Colourless Hybrid Mana
    (56, '{C/W}'),
    (57, '{C/U}'),
    (58, '{C/B}'),
    (59, '{C/R}'),
    (60, '{C/G}'),
-- 2-Generic Hybrid Mana
    (61, '{2/W}'),
    (62, '{2/U}'),
    (63, '{2/B}'),
    (64, '{2/R}'),
    (65, '{2/G}'),
-- Funny & Misc.
    (66, '{S}'),
    (67, '{Y}'),
    (68, '{Z}'),
    (69, '{100}'),
    (70, '{1000000}'),
    (71, '{inf}'),
    (72, '{half}'),
    (73, '{HW}'),
    (74, '{HR}');

-- -----------------------------------------------------
-- supertypes table data
-- -----------------------------------------------------
DELETE FROM supertypes;
INSERT INTO supertypes (id, name) VALUES 
    (1, 'Basic'),
    (2, 'Legendary'),
    (3, 'Snow'),
    (4, 'Host'),
    (5, 'Ongoing'),
    (6, 'World');

-- -----------------------------------------------------
-- types table data
-- -----------------------------------------------------
DELETE FROM types;
INSERT INTO types (id, name) VALUES
    (1, 'Artifact'),
    (2, 'Creature'),
    (3, 'Enchantment'),
    (4, 'Instant'),
    (5, 'Land'),
    (6, 'Planeswalker'),
    (7, 'Sorcery'),
    (8, 'Battle'),
    (9, 'Kindred'),
    (10, 'Conspiracy'),
    (11, 'Dungeon'),
    (12, 'Eaturecray'),
    (13, 'Phenomenon'),
    (14, 'Plane'),
    (15, 'Scheme'),
    (16, 'Summon'),
    (17, 'Vanguard');

    