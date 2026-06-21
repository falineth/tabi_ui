use std::collections::HashSet;

use dioxus::prelude::*;
use tabi_ui::components::*;
use tabi_ui::icons::*;

const TAILWIND_CSS: Asset = asset!("../assets/tailwind.css");

static ICON_LIST: [&str; 1502] = [
    "Md1k",
    "Md1kPlus",
    "Md2k",
    "Md2kPlus",
    "Md2mp",
    "Md3dRotation",
    "Md3k",
    "Md3kPlus",
    "Md3mp",
    "Md4k",
    "Md4kPlus",
    "Md4mp",
    "Md5g",
    "Md5k",
    "Md5kPlus",
    "Md5mp",
    "Md6FtApart",
    "Md6k",
    "Md6kPlus",
    "Md6mp",
    "Md7k",
    "Md7kPlus",
    "Md7mp",
    "Md8k",
    "Md8kPlus",
    "Md8mp",
    "Md9k",
    "Md9kPlus",
    "Md9mp",
    "Md10k",
    "Md10mp",
    "Md11mp",
    "Md12mp",
    "Md13mp",
    "Md14mp",
    "Md15mp",
    "Md16mp",
    "Md17mp",
    "Md18mp",
    "Md19mp",
    "Md20mp",
    "Md21mp",
    "Md22mp",
    "Md23mp",
    "Md24mp",
    "Md360",
    "MdAccessAlarm",
    "MdAccessAlarms",
    "MdAccessibility",
    "MdAccessible",
    "MdAccessibleForward",
    "MdAccessTime",
    "MdAccountBalance",
    "MdAccountBalanceWallet",
    "MdAccountBox",
    "MdAccountCircle",
    "MdAccountTree",
    "MdAcUnit",
    "MdAdb",
    "MdAdd",
    "MdAddAlarm",
    "MdAddAlert",
    "MdAddAPhoto",
    "MdAddBox",
    "MdAddBusiness",
    "MdAddCall",
    "MdAddchart",
    "MdAddChart",
    "MdAddCircle",
    "MdAddCircleOutline",
    "MdAddComment",
    "MdAddIcCall",
    "MdAddLink",
    "MdAddLocation",
    "MdAddLocationAlt",
    "MdAddModerator",
    "MdAddPhotoAlternate",
    "MdAddRoad",
    "MdAddShoppingCart",
    "MdAddTask",
    "MdAddToDrive",
    "MdAddToHomeScreen",
    "MdAddToPhotos",
    "MdAddToQueue",
    "MdAdjust",
    "MdAdminPanelSettings",
    "MdAdUnits",
    "MdAgriculture",
    "MdAirlineSeatFlat",
    "MdAirlineSeatFlatAngled",
    "MdAirlineSeatIndividualSuite",
    "MdAirlineSeatLegroomExtra",
    "MdAirlineSeatLegroomNormal",
    "MdAirlineSeatLegroomReduced",
    "MdAirlineSeatReclineExtra",
    "MdAirlineSeatReclineNormal",
    "MdAirplanemodeActive",
    "MdAirplanemodeInactive",
    "MdAirplay",
    "MdAirportShuttle",
    "MdAlarm",
    "MdAlarmAdd",
    "MdAlarmOff",
    "MdAlarmOn",
    "MdAlbum",
    "MdAllInbox",
    "MdAllInclusive",
    "MdAllOut",
    "MdAlternateEmail",
    "MdAltRoute",
    "MdAmpStories",
    "MdAnalytics",
    "MdAnchor",
    "MdAndroid",
    "MdAnimation",
    "MdAnnouncement",
    "MdApartment",
    "MdApi",
    "MdAppBlocking",
    "MdAppRegistration",
    "MdApproval",
    "MdApps",
    "MdAppSettingsAlt",
    "MdArchitecture",
    "MdArchive",
    "MdArrowBack",
    "MdArrowBackIos",
    "MdArrowCircleDown",
    "MdArrowCircleUp",
    "MdArrowDownward",
    "MdArrowDropDown",
    "MdArrowDropDownCircle",
    "MdArrowDropUp",
    "MdArrowForward",
    "MdArrowForwardIos",
    "MdArrowLeft",
    "MdArrowRight",
    "MdArrowRightAlt",
    "MdArrowUpward",
    "MdArticle",
    "MdArtTrack",
    "MdAspectRatio",
    "MdAssessment",
    "MdAssignment",
    "MdAssignmentInd",
    "MdAssignmentLate",
    "MdAssignmentReturn",
    "MdAssignmentReturned",
    "MdAssignmentTurnedIn",
    "MdAssistant",
    "MdAssistantDirection",
    "MdAssistantNavigation",
    "MdAssistantPhoto",
    "MdAtm",
    "MdAttachEmail",
    "MdAttachFile",
    "MdAttachment",
    "MdAttachMoney",
    "MdAttractions",
    "MdAudiotrack",
    "MdAutoAwesome",
    "MdAutoAwesomeMosaic",
    "MdAutoAwesomeMotion",
    "MdAutoDelete",
    "MdAutoFixHigh",
    "MdAutoFixNormal",
    "MdAutoFixOff",
    "MdAutorenew",
    "MdAutoStories",
    "MdAvTimer",
    "MdBabyChangingStation",
    "MdBackpack",
    "MdBackspace",
    "MdBackup",
    "MdBackupTable",
    "MdBadge",
    "MdBakeryDining",
    "MdBallot",
    "MdBarChart",
    "MdBatchPrediction",
    "MdBathtub",
    "MdBatteryAlert",
    "MdBatteryChargingFull",
    "MdBatteryFull",
    "MdBatteryStd",
    "MdBatteryUnknown",
    "MdBeachAccess",
    "MdBedtime",
    "MdBeenhere",
    "MdBento",
    "MdBikeScooter",
    "MdBiotech",
    "MdBlock",
    "MdBlockFlipped",
    "MdBluetooth",
    "MdBluetoothAudio",
    "MdBluetoothConnected",
    "MdBluetoothDisabled",
    "MdBluetoothSearching",
    "MdBlurCircular",
    "MdBlurLinear",
    "MdBlurOff",
    "MdBlurOn",
    "MdBolt",
    "MdBook",
    "MdBookmark",
    "MdBookmarkBorder",
    "MdBookmarks",
    "MdBookOnline",
    "MdBorderAll",
    "MdBorderBottom",
    "MdBorderClear",
    "MdBorderColor",
    "MdBorderHorizontal",
    "MdBorderInner",
    "MdBorderLeft",
    "MdBorderOuter",
    "MdBorderRight",
    "MdBorderStyle",
    "MdBorderTop",
    "MdBorderVertical",
    "MdBrandingWatermark",
    "MdBreakfastDining",
    "MdBrightness1",
    "MdBrightness2",
    "MdBrightness3",
    "MdBrightness4",
    "MdBrightness5",
    "MdBrightness6",
    "MdBrightness7",
    "MdBrightnessAuto",
    "MdBrightnessHigh",
    "MdBrightnessLow",
    "MdBrightnessMedium",
    "MdBrokenImage",
    "MdBrowserNotSupported",
    "MdBrunchDining",
    "MdBrush",
    "MdBubbleChart",
    "MdBugReport",
    "MdBuild",
    "MdBuildCircle",
    "MdBurstMode",
    "MdBusAlert",
    "MdBusiness",
    "MdBusinessCenter",
    "MdCached",
    "MdCake",
    "MdCalculate",
    "MdCalendarToday",
    "MdCalendarViewDay",
    "MdCall",
    "MdCallEnd",
    "MdCallMade",
    "MdCallMerge",
    "MdCallMissed",
    "MdCallMissedOutgoing",
    "MdCallReceived",
    "MdCallSplit",
    "MdCallToAction",
    "MdCamera",
    "MdCameraAlt",
    "MdCameraEnhance",
    "MdCameraFront",
    "MdCameraRear",
    "MdCameraRoll",
    "MdCampaign",
    "MdCancel",
    "MdCancelPresentation",
    "MdCancelScheduleSend",
    "MdCardGiftcard",
    "MdCardMembership",
    "MdCardTravel",
    "MdCarpenter",
    "MdCarRental",
    "MdCarRepair",
    "MdCases",
    "MdCasino",
    "MdCast",
    "MdCastConnected",
    "MdCastForEducation",
    "MdCategory",
    "MdCelebration",
    "MdCellWifi",
    "MdCenterFocusStrong",
    "MdCenterFocusWeak",
    "MdChangeHistory",
    "MdChargingStation",
    "MdChat",
    "MdChatBubble",
    "MdChatBubbleOutline",
    "MdCheck",
    "MdCheckBox",
    "MdCheckBoxOutlineBlank",
    "MdCheckCircle",
    "MdCheckCircleOutline",
    "MdCheckroom",
    "MdChevronLeft",
    "MdChevronRight",
    "MdChildCare",
    "MdChildFriendly",
    "MdChromeReaderMode",
    "MdCircle",
    "MdCircleNotifications",
    "MdClass",
    "MdCleanHands",
    "MdCleaningServices",
    "MdClear",
    "MdClearAll",
    "MdClose",
    "MdClosedCaption",
    "MdClosedCaptionDisabled",
    "MdClosedCaptionOff",
    "MdCloseFullscreen",
    "MdCloud",
    "MdCloudCircle",
    "MdCloudDone",
    "MdCloudDownload",
    "MdCloudOff",
    "MdCloudQueue",
    "MdCloudUpload",
    "MdCode",
    "MdCollections",
    "MdCollectionsBookmark",
    "MdColorize",
    "MdColorLens",
    "MdComment",
    "MdCommentBank",
    "MdCommute",
    "MdCompare",
    "MdCompareArrows",
    "MdCompassCalibration",
    "MdCompress",
    "MdComputer",
    "MdConfirmationNumber",
    "MdConnectedTv",
    "MdConnectWithoutContact",
    "MdConstruction",
    "MdContactless",
    "MdContactMail",
    "MdContactPage",
    "MdContactPhone",
    "MdContacts",
    "MdContactSupport",
    "MdContentCopy",
    "MdContentCut",
    "MdContentPaste",
    "MdControlCamera",
    "MdControlPoint",
    "MdControlPointDuplicate",
    "MdCopyright",
    "MdCoronavirus",
    "MdCorporateFare",
    "MdCountertops",
    "MdCreate",
    "MdCreateNewFolder",
    "MdCreditCard",
    "MdCrop",
    "MdCrop32",
    "MdCrop54",
    "MdCrop75",
    "MdCrop169",
    "MdCropDin",
    "MdCropFree",
    "MdCropLandscape",
    "MdCropOriginal",
    "MdCropPortrait",
    "MdCropRotate",
    "MdCropSquare",
    "MdDangerous",
    "MdDashboard",
    "MdDashboardCustomize",
    "MdDataUsage",
    "MdDateRange",
    "MdDeck",
    "MdDehaze",
    "MdDelete",
    "MdDeleteForever",
    "MdDeleteOutline",
    "MdDeleteSweep",
    "MdDeliveryDining",
    "MdDepartureBoard",
    "MdDescription",
    "MdDesignServices",
    "MdDesktopAccessDisabled",
    "MdDesktopMac",
    "MdDesktopWindows",
    "MdDetails",
    "MdDeveloperBoard",
    "MdDeveloperMode",
    "MdDeviceHub",
    "MdDevices",
    "MdDevicesOther",
    "MdDeviceThermostat",
    "MdDeviceUnknown",
    "MdDialerSip",
    "MdDialpad",
    "MdDinnerDining",
    "MdDirections",
    "MdDirectionsBike",
    "MdDirectionsBoat",
    "MdDirectionsBus",
    "MdDirectionsCar",
    "MdDirectionsOff",
    "MdDirectionsRailway",
    "MdDirectionsRun",
    "MdDirectionsSubway",
    "MdDirectionsTransit",
    "MdDirectionsWalk",
    "MdDirtyLens",
    "MdDisabledByDefault",
    "MdDiscFull",
    "MdDns",
    "MdDock",
    "MdDomain",
    "MdDomainDisabled",
    "MdDomainVerification",
    "MdDone",
    "MdDoneAll",
    "MdDoneOutline",
    "MdDoNotDisturb",
    "MdDoNotDisturbAlt",
    "MdDoNotDisturbOff",
    "MdDoNotDisturbOn",
    "MdDoNotStep",
    "MdDoNotTouch",
    "MdDonutLarge",
    "MdDonutSmall",
    "MdDoubleArrow",
    "MdDrafts",
    "MdDragHandle",
    "MdDragIndicator",
    "MdDriveEta",
    "MdDriveFileMove",
    "MdDriveFileMoveOutline",
    "MdDriveFileRenameOutline",
    "MdDriveFolderUpload",
    "MdDry",
    "MdDryCleaning",
    "MdDuo",
    "MdDvr",
    "MdDynamicFeed",
    "MdDynamicForm",
    "MdEast",
    "MdEco",
    "MdEdit",
    "MdEditAttributes",
    "MdEditLocation",
    "MdEditOff",
    "MdEditRoad",
    "MdEject",
    "MdElderly",
    "MdElectricalServices",
    "MdElectricBike",
    "MdElectricCar",
    "MdElectricMoped",
    "MdElectricRickshaw",
    "MdElectricScooter",
    "MdElevator",
    "MdEmail",
    "MdEmojiEmotions",
    "MdEmojiEvents",
    "MdEmojiFlags",
    "MdEmojiFoodBeverage",
    "MdEmojiNature",
    "MdEmojiObjects",
    "MdEmojiPeople",
    "MdEmojiSymbols",
    "MdEmojiTransportation",
    "MdEngineering",
    "MdEnhancedEncryption",
    "MdEqualizer",
    "MdError",
    "MdErrorOutline",
    "MdEscalator",
    "MdEscalatorWarning",
    "MdEuro",
    "MdEuroSymbol",
    "MdEvent",
    "MdEventAvailable",
    "MdEventBusy",
    "MdEventNote",
    "MdEventSeat",
    "MdEvStation",
    "MdExitToApp",
    "MdExpand",
    "MdExpandLess",
    "MdExpandMore",
    "MdExplicit",
    "MdExplore",
    "MdExploreOff",
    "MdExposure",
    "MdExposureNeg1",
    "MdExposureNeg2",
    "MdExposurePlus1",
    "MdExposurePlus2",
    "MdExposureZero",
    "MdExtension",
    "MdFace",
    "MdFacebook",
    "MdFaceRetouchingNatural",
    "MdFactCheck",
    "MdFamilyRestroom",
    "MdFastfood",
    "MdFastForward",
    "MdFastRewind",
    "MdFavorite",
    "MdFavoriteBorder",
    "MdFeaturedPlayList",
    "MdFeaturedVideo",
    "MdFeedback",
    "MdFence",
    "MdFestival",
    "MdFiberDvr",
    "MdFiberManualRecord",
    "MdFiberNew",
    "MdFiberPin",
    "MdFiberSmartRecord",
    "MdFileCopy",
    "MdFileDownload",
    "MdFileDownloadDone",
    "MdFilePresent",
    "MdFileUpload",
    "MdFilter",
    "MdFilter1",
    "MdFilter2",
    "MdFilter3",
    "MdFilter4",
    "MdFilter5",
    "MdFilter6",
    "MdFilter7",
    "MdFilter8",
    "MdFilter9",
    "MdFilter9Plus",
    "MdFilterAlt",
    "MdFilterBAndW",
    "MdFilterCenterFocus",
    "MdFilterDrama",
    "MdFilterFrames",
    "MdFilterHdr",
    "MdFilterList",
    "MdFilterListAlt",
    "MdFilterNone",
    "MdFilterTiltShift",
    "MdFilterVintage",
    "MdFindInPage",
    "MdFindReplace",
    "MdFingerprint",
    "MdFireExtinguisher",
    "MdFireplace",
    "MdFirstPage",
    "MdFitnessCenter",
    "MdFitScreen",
    "MdFlag",
    "MdFlaky",
    "MdFlare",
    "MdFlashAuto",
    "MdFlashOff",
    "MdFlashOn",
    "MdFlight",
    "MdFlightLand",
    "MdFlightTakeoff",
    "MdFlip",
    "MdFlipCameraAndroid",
    "MdFlipCameraIos",
    "MdFlipToBack",
    "MdFlipToFront",
    "MdFolder",
    "MdFolderOpen",
    "MdFolderShared",
    "MdFolderSpecial",
    "MdFollowTheSigns",
    "MdFontDownload",
    "MdFoodBank",
    "MdFormatAlignCenter",
    "MdFormatAlignJustify",
    "MdFormatAlignLeft",
    "MdFormatAlignRight",
    "MdFormatBold",
    "MdFormatClear",
    "MdFormatColorFill",
    "MdFormatColorReset",
    "MdFormatColorText",
    "MdFormatIndentDecrease",
    "MdFormatIndentIncrease",
    "MdFormatItalic",
    "MdFormatLineSpacing",
    "MdFormatListBulleted",
    "MdFormatListNumbered",
    "MdFormatListNumberedRtl",
    "MdFormatPaint",
    "MdFormatQuote",
    "MdFormatShapes",
    "MdFormatSize",
    "MdFormatStrikethrough",
    "MdFormatTextdirectionLToR",
    "MdFormatTextdirectionRToL",
    "MdFormatUnderlined",
    "MdForum",
    "MdForward",
    "MdForward5",
    "MdForward10",
    "MdForward30",
    "MdForwardToInbox",
    "MdFoundation",
    "MdFreeBreakfast",
    "MdFullscreen",
    "MdFullscreenExit",
    "MdFunctions",
    "MdGamepad",
    "MdGames",
    "MdGavel",
    "MdGesture",
    "MdGetApp",
    "MdGif",
    "MdGolfCourse",
    "MdGpsFixed",
    "MdGpsNotFixed",
    "MdGpsOff",
    "MdGrade",
    "MdGradient",
    "MdGrading",
    "MdGrain",
    "MdGraphicEq",
    "MdGrass",
    "MdGridOff",
    "MdGridOn",
    "MdGridView",
    "MdGroup",
    "MdGroupAdd",
    "MdGroups",
    "MdGroupWork",
    "MdGTranslate",
    "MdHail",
    "MdHandyman",
    "MdHardware",
    "MdHd",
    "MdHdrEnhancedSelect",
    "MdHdrOff",
    "MdHdrOn",
    "MdHdrStrong",
    "MdHdrWeak",
    "MdHeadset",
    "MdHeadsetMic",
    "MdHeadsetOff",
    "MdHealing",
    "MdHearing",
    "MdHearingDisabled",
    "MdHeight",
    "MdHelp",
    "MdHelpCenter",
    "MdHelpOutline",
    "MdHighlight",
    "MdHighlightAlt",
    "MdHighlightOff",
    "MdHighQuality",
    "MdHistory",
    "MdHistoryEdu",
    "MdHistoryToggleOff",
    "MdHome",
    "MdHomeFilled",
    "MdHomeRepairService",
    "MdHomeWork",
    "MdHorizontalRule",
    "MdHorizontalSplit",
    "MdHotel",
    "MdHotTub",
    "MdHourglassBottom",
    "MdHourglassDisabled",
    "MdHourglassEmpty",
    "MdHourglassFull",
    "MdHourglassTop",
    "MdHouse",
    "MdHouseSiding",
    "MdHowToReg",
    "MdHowToVote",
    "MdHttp",
    "MdHttps",
    "MdHvac",
    "MdIcecream",
    "MdImage",
    "MdImageAspectRatio",
    "MdImageNotSupported",
    "MdImageSearch",
    "MdImagesearchRoller",
    "MdImportantDevices",
    "MdImportContacts",
    "MdImportExport",
    "MdInbox",
    "MdIndeterminateCheckBox",
    "MdInfo",
    "MdInfoOutline",
    "MdInput",
    "MdInsertChart",
    "MdInsertChartOutlined",
    "MdInsertComment",
    "MdInsertDriveFile",
    "MdInsertEmoticon",
    "MdInsertInvitation",
    "MdInsertLink",
    "MdInsertPhoto",
    "MdInsights",
    "MdIntegrationInstructions",
    "MdInventory",
    "MdInvertColors",
    "MdInvertColorsOff",
    "MdIosShare",
    "MdIso",
    "MdKeyboard",
    "MdKeyboardArrowDown",
    "MdKeyboardArrowLeft",
    "MdKeyboardArrowRight",
    "MdKeyboardArrowUp",
    "MdKeyboardBackspace",
    "MdKeyboardCapslock",
    "MdKeyboardHide",
    "MdKeyboardReturn",
    "MdKeyboardTab",
    "MdKeyboardVoice",
    "MdKingBed",
    "MdKitchen",
    "MdLabel",
    "MdLabelImportant",
    "MdLabelImportantOutline",
    "MdLabelOff",
    "MdLabelOutline",
    "MdLandscape",
    "MdLanguage",
    "MdLaptop",
    "MdLaptopChromebook",
    "MdLaptopMac",
    "MdLaptopWindows",
    "MdLastPage",
    "MdLaunch",
    "MdLayers",
    "MdLayersClear",
    "MdLeaderboard",
    "MdLeakAdd",
    "MdLeakRemove",
    "MdLegendToggle",
    "MdLens",
    "MdLibraryAdd",
    "MdLibraryAddCheck",
    "MdLibraryBooks",
    "MdLibraryMusic",
    "MdLightbulb",
    "MdLightbulbOutline",
    "MdLinearScale",
    "MdLineStyle",
    "MdLineWeight",
    "MdLink",
    "MdLinkedCamera",
    "MdLinkOff",
    "MdLiquor",
    "MdList",
    "MdListAlt",
    "MdLiveHelp",
    "MdLiveTv",
    "MdLocalActivity",
    "MdLocalAirport",
    "MdLocalAtm",
    "MdLocalBar",
    "MdLocalCafe",
    "MdLocalCarWash",
    "MdLocalConvenienceStore",
    "MdLocalDining",
    "MdLocalDrink",
    "MdLocalFireDepartment",
    "MdLocalFlorist",
    "MdLocalGasStation",
    "MdLocalGroceryStore",
    "MdLocalHospital",
    "MdLocalHotel",
    "MdLocalLaundryService",
    "MdLocalLibrary",
    "MdLocalMall",
    "MdLocalMovies",
    "MdLocalOffer",
    "MdLocalParking",
    "MdLocalPharmacy",
    "MdLocalPhone",
    "MdLocalPizza",
    "MdLocalPlay",
    "MdLocalPolice",
    "MdLocalPostOffice",
    "MdLocalPrintshop",
    "MdLocalSee",
    "MdLocalShipping",
    "MdLocalTaxi",
    "MdLocationCity",
    "MdLocationDisabled",
    "MdLocationOff",
    "MdLocationOn",
    "MdLocationPin",
    "MdLocationSearching",
    "MdLock",
    "MdLockClock",
    "MdLockOpen",
    "MdLockOutline",
    "MdLogin",
    "MdLogout",
    "MdLooks",
    "MdLooks3",
    "MdLooks4",
    "MdLooks5",
    "MdLooks6",
    "MdLooksOne",
    "MdLooksTwo",
    "MdLoop",
    "MdLoupe",
    "MdLowPriority",
    "MdLoyalty",
    "MdLuggage",
    "MdLunchDining",
    "MdMail",
    "MdMailOutline",
    "MdMap",
    "MdMapsUgc",
    "MdMargin",
    "MdMarkAsUnread",
    "MdMarkChatRead",
    "MdMarkChatUnread",
    "MdMarkEmailRead",
    "MdMarkEmailUnread",
    "MdMarkunread",
    "MdMarkunreadMailbox",
    "MdMasks",
    "MdMaximize",
    "MdMediation",
    "MdMedicalServices",
    "MdMeetingRoom",
    "MdMemory",
    "MdMenu",
    "MdMenuBook",
    "MdMenuOpen",
    "MdMergeType",
    "MdMessage",
    "MdMic",
    "MdMicExternalOff",
    "MdMicExternalOn",
    "MdMicNone",
    "MdMicOff",
    "MdMicrowave",
    "MdMilitaryTech",
    "MdMinimize",
    "MdMiscellaneousServices",
    "MdMissedVideoCall",
    "MdMms",
    "MdMobileFriendly",
    "MdMobileOff",
    "MdMobileScreenShare",
    "MdModeComment",
    "MdModeEdit",
    "MdModelTraining",
    "MdMonetizationOn",
    "MdMoney",
    "MdMoneyOff",
    "MdMonitor",
    "MdMonochromePhotos",
    "MdMood",
    "MdMoodBad",
    "MdMoped",
    "MdMore",
    "MdMoreHoriz",
    "MdMoreTime",
    "MdMoreVert",
    "MdMotionPhotosOff",
    "MdMotionPhotosOn",
    "MdMotionPhotosPause",
    "MdMotionPhotosPaused",
    "MdMouse",
    "MdMoveToInbox",
    "MdMovie",
    "MdMovieCreation",
    "MdMovieFilter",
    "MdMp",
    "MdMultilineChart",
    "MdMultipleStop",
    "MdMuseum",
    "MdMusicNote",
    "MdMusicOff",
    "MdMusicVideo",
    "MdMyLocation",
    "MdNat",
    "MdNature",
    "MdNaturePeople",
    "MdNavigateBefore",
    "MdNavigateNext",
    "MdNavigation",
    "MdNearMe",
    "MdNearMeDisabled",
    "MdNetworkCell",
    "MdNetworkCheck",
    "MdNetworkLocked",
    "MdNetworkWifi",
    "MdNewReleases",
    "MdNextPlan",
    "MdNextWeek",
    "MdNfc",
    "MdNightlife",
    "MdNightlightRound",
    "MdNightShelter",
    "MdNightsStay",
    "MdNoBackpack",
    "MdNoCell",
    "MdNoDrinks",
    "MdNoEncryption",
    "MdNoFlash",
    "MdNoFood",
    "MdNoLuggage",
    "MdNoMeals",
    "MdNoMealsOuline",
    "MdNoMeetingRoom",
    "MdNoPhotography",
    "MdNorth",
    "MdNorthEast",
    "MdNorthWest",
    "MdNoSim",
    "MdNoStroller",
    "MdNotAccessible",
    "MdNote",
    "MdNoteAdd",
    "MdNotes",
    "MdNotificationImportant",
    "MdNotifications",
    "MdNotificationsActive",
    "MdNotificationsNone",
    "MdNotificationsOff",
    "MdNotificationsPaused",
    "MdNotInterested",
    "MdNotListedLocation",
    "MdNoTransfer",
    "MdNotStarted",
    "MdOfflineBolt",
    "MdOfflinePin",
    "MdOfflineShare",
    "MdOndemandVideo",
    "MdOnlinePrediction",
    "MdOpacity",
    "MdOpenInBrowser",
    "MdOpenInFull",
    "MdOpenInNew",
    "MdOpenWith",
    "MdOutbond",
    "MdOutbox",
    "MdOutdoorGrill",
    "MdOutgoingMail",
    "MdOutlet",
    "MdOutlinedFlag",
    "MdPadding",
    "MdPages",
    "MdPageview",
    "MdPalette",
    "MdPanorama",
    "MdPanoramaFishEye",
    "MdPanoramaHorizontal",
    "MdPanoramaHorizontalSelect",
    "MdPanoramaPhotosphere",
    "MdPanoramaPhotosphereSelect",
    "MdPanoramaVertical",
    "MdPanoramaVerticalSelect",
    "MdPanoramaWideAngle",
    "MdPanoramaWideAngleSelect",
    "MdPanTool",
    "MdPark",
    "MdPartyMode",
    "MdPause",
    "MdPauseCircleFilled",
    "MdPauseCircleOutline",
    "MdPausePresentation",
    "MdPayment",
    "MdPayments",
    "MdPedalBike",
    "MdPending",
    "MdPendingActions",
    "MdPeople",
    "MdPeopleAlt",
    "MdPeopleOutline",
    "MdPermCameraMic",
    "MdPermContactCalendar",
    "MdPermDataSetting",
    "MdPermDeviceInformation",
    "MdPermIdentity",
    "MdPermMedia",
    "MdPermPhoneMsg",
    "MdPermScanWifi",
    "MdPerson",
    "MdPersonAdd",
    "MdPersonAddAlt",
    "MdPersonAddAlt1",
    "MdPersonAddDisabled",
    "MdPersonalVideo",
    "MdPersonOutline",
    "MdPersonPin",
    "MdPersonPinCircle",
    "MdPersonRemove",
    "MdPersonRemoveAlt1",
    "MdPersonSearch",
    "MdPestControl",
    "MdPestControlRodent",
    "MdPets",
    "MdPhone",
    "MdPhoneAndroid",
    "MdPhoneBluetoothSpeaker",
    "MdPhoneCallback",
    "MdPhoneDisabled",
    "MdPhoneEnabled",
    "MdPhoneForwarded",
    "MdPhoneInTalk",
    "MdPhoneIphone",
    "MdPhonelink",
    "MdPhonelinkErase",
    "MdPhonelinkLock",
    "MdPhonelinkOff",
    "MdPhonelinkRing",
    "MdPhonelinkSetup",
    "MdPhoneLocked",
    "MdPhoneMissed",
    "MdPhonePaused",
    "MdPhoto",
    "MdPhotoAlbum",
    "MdPhotoCamera",
    "MdPhotoCameraBack",
    "MdPhotoCameraFront",
    "MdPhotoFilter",
    "MdPhotoLibrary",
    "MdPhotoSizeSelectActual",
    "MdPhotoSizeSelectLarge",
    "MdPhotoSizeSelectSmall",
    "MdPictureAsPdf",
    "MdPictureInPicture",
    "MdPictureInPictureAlt",
    "MdPieChart",
    "MdPieChartOutlined",
    "MdPinDrop",
    "MdPivotTableChart",
    "MdPlace",
    "MdPlagiarism",
    "MdPlayArrow",
    "MdPlayCircleFilled",
    "MdPlayCircleOutline",
    "MdPlayDisabled",
    "MdPlayForWork",
    "MdPlaylistAdd",
    "MdPlaylistAddCheck",
    "MdPlaylistPlay",
    "MdPlumbing",
    "MdPlusOne",
    "MdPointOfSale",
    "MdPolicy",
    "MdPoll",
    "MdPolymer",
    "MdPool",
    "MdPortableWifiOff",
    "MdPortrait",
    "MdPostAdd",
    "MdPower",
    "MdPowerInput",
    "MdPowerOff",
    "MdPowerSettingsNew",
    "MdPregnantWoman",
    "MdPresentToAll",
    "MdPreview",
    "MdPrint",
    "MdPrintDisabled",
    "MdPriorityHigh",
    "MdPrivacyTip",
    "MdPsychology",
    "MdPublic",
    "MdPublicOff",
    "MdPublish",
    "MdPublishedWithChanges",
    "MdPushPin",
    "MdQrCode",
    "MdQrCodeScanner",
    "MdQueryBuilder",
    "MdQuestionAnswer",
    "MdQueue",
    "MdQueueMusic",
    "MdQueuePlayNext",
    "MdQuickreply",
    "MdRadio",
    "MdRadioButtonChecked",
    "MdRadioButtonUnchecked",
    "MdRailwayAlert",
    "MdRamenDining",
    "MdRateReview",
    "MdReadMore",
    "MdReceipt",
    "MdReceiptLong",
    "MdRecentActors",
    "MdRecommend",
    "MdRecordVoiceOver",
    "MdRedeem",
    "MdRedo",
    "MdReduceCapacity",
    "MdRefresh",
    "MdRemove",
    "MdRemoveCircle",
    "MdRemoveCircleOutline",
    "MdRemoveDone",
    "MdRemoveFromQueue",
    "MdRemoveModerator",
    "MdRemoveRedEye",
    "MdRemoveShoppingCart",
    "MdReorder",
    "MdRepeat",
    "MdRepeatOn",
    "MdRepeatOne",
    "MdRepeatOneOn",
    "MdReplay",
    "MdReplay5",
    "MdReplay10",
    "MdReplay30",
    "MdReplayCircleFilled",
    "MdReply",
    "MdReplyAll",
    "MdReport",
    "MdReportOff",
    "MdReportProblem",
    "MdRequestPage",
    "MdRequestQuote",
    "MdResetTv",
    "MdRestaurant",
    "MdRestaurantMenu",
    "MdRestore",
    "MdRestoreFromTrash",
    "MdRestorePage",
    "MdRiceBowl",
    "MdRingVolume",
    "MdRoofing",
    "MdRoom",
    "MdRoomPreferences",
    "MdRoomService",
    "MdRotate90DegreesCcw",
    "MdRotateLeft",
    "MdRotateRight",
    "MdRoundedCorner",
    "MdRouter",
    "MdRowing",
    "MdRssFeed",
    "MdRtt",
    "MdRule",
    "MdRuleFolder",
    "MdRunCircle",
    "MdRvHookup",
    "MdSanitizer",
    "MdSatellite",
    "MdSave",
    "MdSaveAlt",
    "MdSavedSearch",
    "MdScanner",
    "MdScatterPlot",
    "MdSchedule",
    "MdScheduleSend",
    "MdSchool",
    "MdScience",
    "MdScore",
    "MdScreenLockLandscape",
    "MdScreenLockPortrait",
    "MdScreenLockRotation",
    "MdScreenRotation",
    "MdScreenSearchDesktop",
    "MdScreenShare",
    "MdSd",
    "MdSdCard",
    "MdSdStorage",
    "MdSearch",
    "MdSearchOff",
    "MdSecurity",
    "MdSegment",
    "MdSelectAll",
    "MdSelfImprovement",
    "MdSend",
    "MdSendAndArchive",
    "MdSendToMobile",
    "MdSensorDoor",
    "MdSensorWindow",
    "MdSentimentDissatisfied",
    "MdSentimentNeutral",
    "MdSentimentSatisfied",
    "MdSentimentSatisfiedAlt",
    "MdSentimentVeryDissatisfied",
    "MdSentimentVerySatisfied",
    "MdSetMeal",
    "MdSettings",
    "MdSettingsApplications",
    "MdSettingsBackupRestore",
    "MdSettingsBluetooth",
    "MdSettingsBrightness",
    "MdSettingsCell",
    "MdSettingsEthernet",
    "MdSettingsInputAntenna",
    "MdSettingsInputComponent",
    "MdSettingsInputComposite",
    "MdSettingsInputHdmi",
    "MdSettingsInputSvideo",
    "MdSettingsOverscan",
    "MdSettingsPhone",
    "MdSettingsPower",
    "MdSettingsRemote",
    "MdSettingsSystemDaydream",
    "MdSettingsVoice",
    "MdShare",
    "MdShield",
    "MdShop",
    "MdShoppingBag",
    "MdShoppingBasket",
    "MdShoppingCart",
    "MdShopTwo",
    "MdShortText",
    "MdShowChart",
    "MdShuffle",
    "MdShuffleOn",
    "MdShutterSpeed",
    "MdSick",
    "MdSignalCellular0Bar",
    "MdSignalCellular4Bar",
    "MdSignalCellularAlt",
    "MdSignalCellularConnectedNoInternet4Bar",
    "MdSignalCellularNoSim",
    "MdSignalCellularNull",
    "MdSignalCellularOff",
    "MdSignalWifi0Bar",
    "MdSignalWifi4Bar",
    "MdSignalWifi4BarLock",
    "MdSignalWifiOff",
    "MdSimCard",
    "MdSimCardAlert",
    "MdSingleBed",
    "MdSkipNext",
    "MdSkipPrevious",
    "MdSlideshow",
    "MdSlowMotionVideo",
    "MdSmartButton",
    "MdSmartphone",
    "MdSmokeFree",
    "MdSmokingRooms",
    "MdSms",
    "MdSmsFailed",
    "MdSnippetFolder",
    "MdSnooze",
    "MdSoap",
    "MdSort",
    "MdSortByAlpha",
    "MdSource",
    "MdSouth",
    "MdSouthEast",
    "MdSouthWest",
    "MdSpa",
    "MdSpaceBar",
    "MdSpeaker",
    "MdSpeakerGroup",
    "MdSpeakerNotes",
    "MdSpeakerNotesOff",
    "MdSpeakerPhone",
    "MdSpeed",
    "MdSpellcheck",
    "MdSports",
    "MdSportsBar",
    "MdSportsBaseball",
    "MdSportsBasketball",
    "MdSportsCricket",
    "MdSportsEsports",
    "MdSportsFootball",
    "MdSportsGolf",
    "MdSportsHandball",
    "MdSportsHockey",
    "MdSportsKabaddi",
    "MdSportsMma",
    "MdSportsMotorsports",
    "MdSportsRugby",
    "MdSportsSoccer",
    "MdSportsTennis",
    "MdSportsVolleyball",
    "MdSquareFoot",
    "MdStackedBarChart",
    "MdStackedLineChart",
    "MdStairs",
    "MdStar",
    "MdStarBorder",
    "MdStarHalf",
    "MdStarOutline",
    "MdStarRate",
    "MdStars",
    "MdStayCurrentLandscape",
    "MdStayCurrentPortrait",
    "MdStayPrimaryLandscape",
    "MdStayPrimaryPortrait",
    "MdStickyNote2",
    "MdStop",
    "MdStopCircle",
    "MdStopScreenShare",
    "MdStorage",
    "MdStore",
    "MdStorefront",
    "MdStoreMallDirectory",
    "MdStraighten",
    "MdStream",
    "MdStreetview",
    "MdStrikethroughS",
    "MdStroller",
    "MdStyle",
    "MdSubdirectoryArrowLeft",
    "MdSubdirectoryArrowRight",
    "MdSubject",
    "MdSubscript",
    "MdSubscriptions",
    "MdSubtitles",
    "MdSubtitlesOff",
    "MdSubway",
    "MdSuperscript",
    "MdSupervisedUserCircle",
    "MdSupervisorAccount",
    "MdSupport",
    "MdSupportAgent",
    "MdSurroundSound",
    "MdSwapCalls",
    "MdSwapHoriz",
    "MdSwapHorizontalCircle",
    "MdSwapVert",
    "MdSwapVerticalCircle",
    "MdSwipe",
    "MdSwitchAccount",
    "MdSwitchCamera",
    "MdSwitchLeft",
    "MdSwitchRight",
    "MdSwitchVideo",
    "MdSync",
    "MdSyncAlt",
    "MdSyncDisabled",
    "MdSyncProblem",
    "MdSystemUpdate",
    "MdSystemUpdateAlt",
    "MdTab",
    "MdTableChart",
    "MdTableRows",
    "MdTablet",
    "MdTabletAndroid",
    "MdTabletMac",
    "MdTableView",
    "MdTabUnselected",
    "MdTag",
    "MdTagFaces",
    "MdTakeoutDining",
    "MdTapAndPlay",
    "MdTapas",
    "MdTaxiAlert",
    "MdTerrain",
    "MdTextFields",
    "MdTextFormat",
    "MdTextRotateUp",
    "MdTextRotateVertical",
    "MdTextRotationAngledown",
    "MdTextRotationAngleup",
    "MdTextRotationDown",
    "MdTextRotationNone",
    "MdTextsms",
    "MdTextSnippet",
    "MdTexture",
    "MdTheaterComedy",
    "MdTheaters",
    "MdThumbDown",
    "MdThumbDownAlt",
    "MdThumbDownOffAlt",
    "MdThumbsUpDown",
    "MdThumbUp",
    "MdThumbUpAlt",
    "MdThumbUpOffAlt",
    "MdTimelapse",
    "MdTimeline",
    "MdTimer",
    "MdTimer3",
    "MdTimer10",
    "MdTimerOff",
    "MdTimeToLeave",
    "MdTitle",
    "MdToc",
    "MdToday",
    "MdToggleOff",
    "MdToggleOn",
    "MdToll",
    "MdTonality",
    "MdTopic",
    "MdTouchApp",
    "MdTour",
    "MdToys",
    "MdTrackChanges",
    "MdTraffic",
    "MdTrain",
    "MdTram",
    "MdTransferWithinAStation",
    "MdTransform",
    "MdTransitEnterexit",
    "MdTranslate",
    "MdTrendingDown",
    "MdTrendingFlat",
    "MdTrendingUp",
    "MdTripOrigin",
    "MdTty",
    "MdTune",
    "MdTurnedIn",
    "MdTurnedInNot",
    "MdTv",
    "MdTvOff",
    "MdTwoWheeler",
    "MdUmbrella",
    "MdUnarchive",
    "MdUndo",
    "MdUnfoldLess",
    "MdUnfoldMore",
    "MdUnpublished",
    "MdUnsubscribe",
    "MdUpdate",
    "MdUpgrade",
    "MdUploadFile",
    "MdUsb",
    "MdVerified",
    "MdVerifiedUser",
    "MdVerticalAlignBottom",
    "MdVerticalAlignCenter",
    "MdVerticalAlignTop",
    "MdVerticalSplit",
    "MdVibration",
    "MdVideoCall",
    "MdVideocam",
    "MdVideocamOff",
    "MdVideogameAsset",
    "MdVideoLabel",
    "MdVideoLibrary",
    "MdVideoSettings",
    "MdViewAgenda",
    "MdViewArray",
    "MdViewCarousel",
    "MdViewColumn",
    "MdViewComfy",
    "MdViewCompact",
    "MdViewDay",
    "MdViewHeadline",
    "MdViewInAr",
    "MdViewList",
    "MdViewModule",
    "MdViewQuilt",
    "MdViewSidebar",
    "MdViewStream",
    "MdViewWeek",
    "MdVignette",
    "MdVisibility",
    "MdVisibilityOff",
    "MdVoiceChat",
    "MdVoicemail",
    "MdVoiceOverOff",
    "MdVolumeDown",
    "MdVolumeMute",
    "MdVolumeOff",
    "MdVolumeUp",
    "MdVolunteerActivism",
    "MdVpnKey",
    "MdVpnLock",
    "MdWallpaper",
    "MdWarning",
    "MdWash",
    "MdWatch",
    "MdWatchLater",
    "MdWaterDamage",
    "MdWaterfallChart",
    "MdWaves",
    "MdWbAuto",
    "MdWbCloudy",
    "MdWbIncandescent",
    "MdWbIridescent",
    "MdWbShade",
    "MdWbSunny",
    "MdWbTwighlight",
    "MdWc",
    "MdWeb",
    "MdWebAsset",
    "MdWeekend",
    "MdWest",
    "MdWhatshot",
    "MdWheelchairPickup",
    "MdWhereToVote",
    "MdWidgets",
    "MdWifi",
    "MdWifiCalling",
    "MdWifiLock",
    "MdWifiOff",
    "MdWifiProtectedSetup",
    "MdWifiTethering",
    "MdWineBar",
    "MdWork",
    "MdWorkOff",
    "MdWorkOutline",
    "MdWorkspacesFilled",
    "MdWorkspacesOutline",
    "MdWrapText",
    "MdWrongLocation",
    "MdWysiwyg",
    "MdYoutubeSearchedFor",
    "MdZoomIn",
    "MdZoomOut",
    "MdZoomOutMap",
];

#[cfg(feature = "desktop")]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    use dioxus::desktop::{Config, WindowBuilder};
    use tabi_ui::ThemeContext;

    let theme_context = ThemeContext::init().await;

    let mut config = Config::default()
        .with_background_color(theme_context.bg_color)
        .with_window(WindowBuilder::new().with_title("Material Icons"));

    if cfg!(not(debug_assertions)) {
        config = config.with_menu(None);
    }

    LaunchBuilder::new()
        .with_cfg(config)
        .with_context(theme_context)
        .launch(App);
}

#[cfg(feature = "web")]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut found_icons: Signal<HashSet<&str>> = use_signal(HashSet::default);

    let handle_search_text_changed = use_callback(move |value: String| {
        let value = value.to_lowercase();

        let search_words = value
            .split(" ")
            .filter(|word| !word.is_empty())
            .collect::<Vec<_>>();

        let mut search_results: HashSet<&str> = HashSet::default();

        'outer: for icon in ICON_LIST {
            let icon_lower_case = icon.to_lowercase();

            if search_words.is_empty() {
                search_results.insert(icon);
                continue;
            }

            for search_word in search_words.iter() {
                if !icon_lower_case.contains(search_word) {
                    continue 'outer;
                }
            }

            search_results.insert(icon);
        }

        found_icons.set(search_results);
    });

    use_future(move || async move { handle_search_text_changed.call(String::default()) });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        TabiDefaultContext { class: "h-lvh",
            div {
                TextInput { on_value_change: handle_search_text_changed }
            }
            div {
                LotA { found_icons }
                LotB { found_icons }
                LotC { found_icons }
                LotD { found_icons }
                LotE { found_icons }
                LotF { found_icons }
                LotG { found_icons }
                LotH { found_icons }
                LotI { found_icons }
                LotJ { found_icons }
                LotK { found_icons }
                LotL { found_icons }
                LotM { found_icons }
                LotN { found_icons }
                LotO { found_icons }
            }
        }
    }
}

#[component]
fn LotA(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("Md1k") {

                div { title: "Md1k",
                    Icon { icon: Md1k, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md1kPlus") {
                div { title: "Md1kPlus",
                    Icon { icon: Md1kPlus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md2k") {
                div { title: "Md2k",
                    Icon { icon: Md2k, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md2kPlus") {
                div { title: "Md2kPlus",
                    Icon { icon: Md2kPlus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md2mp") {
                div { title: "Md2mp",
                    Icon { icon: Md2mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md3dRotation") {
                div { title: "Md3dRotation",
                    Icon { icon: Md3dRotation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md3k") {
                div { title: "Md3k",
                    Icon { icon: Md3k, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md3kPlus") {
                div { title: "Md3kPlus",
                    Icon { icon: Md3kPlus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md3mp") {
                div { title: "Md3mp",
                    Icon { icon: Md3mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md4k") {
                div { title: "Md4k",
                    Icon { icon: Md4k, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md4kPlus") {
                div { title: "Md4kPlus",
                    Icon { icon: Md4kPlus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md4mp") {
                div { title: "Md4mp",
                    Icon { icon: Md4mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md5g") {
                div { title: "Md5g",
                    Icon { icon: Md5g, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md5k") {
                div { title: "Md5k",
                    Icon { icon: Md5k, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md5kPlus") {
                div { title: "Md5kPlus",
                    Icon { icon: Md5kPlus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md5mp") {
                div { title: "Md5mp",
                    Icon { icon: Md5mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md6FtApart") {
                div { title: "Md6FtApart",
                    Icon { icon: Md6FtApart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md6k") {
                div { title: "Md6k",
                    Icon { icon: Md6k, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md6kPlus") {
                div { title: "Md6kPlus",
                    Icon { icon: Md6kPlus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md6mp") {
                div { title: "Md6mp",
                    Icon { icon: Md6mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md7k") {
                div { title: "Md7k",
                    Icon { icon: Md7k, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md7kPlus") {
                div { title: "Md7kPlus",
                    Icon { icon: Md7kPlus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md7mp") {
                div { title: "Md7mp",
                    Icon { icon: Md7mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md8k") {
                div { title: "Md8k",
                    Icon { icon: Md8k, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md8kPlus") {
                div { title: "Md8kPlus",
                    Icon { icon: Md8kPlus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md8mp") {
                div { title: "Md8mp",
                    Icon { icon: Md8mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md9k") {
                div { title: "Md9k",
                    Icon { icon: Md9k, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md9kPlus") {
                div { title: "Md9kPlus",
                    Icon { icon: Md9kPlus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md9mp") {
                div { title: "Md9mp",
                    Icon { icon: Md9mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md10k") {
                div { title: "Md10k",
                    Icon { icon: Md10k, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md10mp") {
                div { title: "Md10mp",
                    Icon { icon: Md10mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md11mp") {
                div { title: "Md11mp",
                    Icon { icon: Md11mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md12mp") {
                div { title: "Md12mp",
                    Icon { icon: Md12mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md13mp") {
                div { title: "Md13mp",
                    Icon { icon: Md13mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md14mp") {
                div { title: "Md14mp",
                    Icon { icon: Md14mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md15mp") {
                div { title: "Md15mp",
                    Icon { icon: Md15mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md16mp") {
                div { title: "Md16mp",
                    Icon { icon: Md16mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md17mp") {
                div { title: "Md17mp",
                    Icon { icon: Md17mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md18mp") {
                div { title: "Md18mp",
                    Icon { icon: Md18mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md19mp") {
                div { title: "Md19mp",
                    Icon { icon: Md19mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md20mp") {
                div { title: "Md20mp",
                    Icon { icon: Md20mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md21mp") {
                div { title: "Md21mp",
                    Icon { icon: Md21mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md22mp") {
                div { title: "Md22mp",
                    Icon { icon: Md22mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md23mp") {
                div { title: "Md23mp",
                    Icon { icon: Md23mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md24mp") {
                div { title: "Md24mp",
                    Icon { icon: Md24mp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("Md360") {
                div { title: "Md360",
                    Icon { icon: Md360, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccessAlarm") {
                div { title: "MdAccessAlarm",
                    Icon { icon: MdAccessAlarm, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccessAlarms") {
                div { title: "MdAccessAlarms",
                    Icon { icon: MdAccessAlarms, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccessibility") {
                div { title: "MdAccessibility",
                    Icon { icon: MdAccessibility, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccessible") {
                div { title: "MdAccessible",
                    Icon { icon: MdAccessible, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccessibleForward") {
                div { title: "MdAccessibleForward",
                    Icon { icon: MdAccessibleForward, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccessTime") {
                div { title: "MdAccessTime",
                    Icon { icon: MdAccessTime, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccountBalance") {
                div { title: "MdAccountBalance",
                    Icon { icon: MdAccountBalance, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccountBalanceWallet") {
                div { title: "MdAccountBalanceWallet",
                    Icon { icon: MdAccountBalanceWallet, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccountBox") {
                div { title: "MdAccountBox",
                    Icon { icon: MdAccountBox, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccountCircle") {
                div { title: "MdAccountCircle",
                    Icon { icon: MdAccountCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAccountTree") {
                div { title: "MdAccountTree",
                    Icon { icon: MdAccountTree, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAcUnit") {
                div { title: "MdAcUnit",
                    Icon { icon: MdAcUnit, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAdb") {
                div { title: "MdAdb",
                    Icon { icon: MdAdb, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAdd") {
                div { title: "MdAdd",
                    Icon { icon: MdAdd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddAlarm") {
                div { title: "MdAddAlarm",
                    Icon { icon: MdAddAlarm, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddAlert") {
                div { title: "MdAddAlert",
                    Icon { icon: MdAddAlert, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddAPhoto") {
                div { title: "MdAddAPhoto",
                    Icon { icon: MdAddAPhoto, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddBox") {
                div { title: "MdAddBox",
                    Icon { icon: MdAddBox, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddBusiness") {
                div { title: "MdAddBusiness",
                    Icon { icon: MdAddBusiness, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddCall") {
                div { title: "MdAddCall",
                    Icon { icon: MdAddCall, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddchart") {
                div { title: "MdAddchart",
                    Icon { icon: MdAddchart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddChart") {
                div { title: "MdAddChart",
                    Icon { icon: MdAddChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddCircle") {
                div { title: "MdAddCircle",
                    Icon { icon: MdAddCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddCircleOutline") {
                div { title: "MdAddCircleOutline",
                    Icon { icon: MdAddCircleOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddComment") {
                div { title: "MdAddComment",
                    Icon { icon: MdAddComment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddIcCall") {
                div { title: "MdAddIcCall",
                    Icon { icon: MdAddIcCall, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddLink") {
                div { title: "MdAddLink",
                    Icon { icon: MdAddLink, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddLocation") {
                div { title: "MdAddLocation",
                    Icon { icon: MdAddLocation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddLocationAlt") {
                div { title: "MdAddLocationAlt",
                    Icon { icon: MdAddLocationAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddModerator") {
                div { title: "MdAddModerator",
                    Icon { icon: MdAddModerator, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddPhotoAlternate") {
                div { title: "MdAddPhotoAlternate",
                    Icon { icon: MdAddPhotoAlternate, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddRoad") {
                div { title: "MdAddRoad",
                    Icon { icon: MdAddRoad, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddShoppingCart") {
                div { title: "MdAddShoppingCart",
                    Icon { icon: MdAddShoppingCart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddTask") {
                div { title: "MdAddTask",
                    Icon { icon: MdAddTask, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddToDrive") {
                div { title: "MdAddToDrive",
                    Icon { icon: MdAddToDrive, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddToHomeScreen") {
                div { title: "MdAddToHomeScreen",
                    Icon { icon: MdAddToHomeScreen, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddToPhotos") {
                div { title: "MdAddToPhotos",
                    Icon { icon: MdAddToPhotos, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAddToQueue") {
                div { title: "MdAddToQueue",
                    Icon { icon: MdAddToQueue, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAdjust") {
                div { title: "MdAdjust",
                    Icon { icon: MdAdjust, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAdminPanelSettings") {
                div { title: "MdAdminPanelSettings",
                    Icon { icon: MdAdminPanelSettings, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAdUnits") {
                div { title: "MdAdUnits",
                    Icon { icon: MdAdUnits, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAgriculture") {
                div { title: "MdAgriculture",
                    Icon { icon: MdAgriculture, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAirlineSeatFlat") {
                div { title: "MdAirlineSeatFlat",
                    Icon { icon: MdAirlineSeatFlat, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAirlineSeatFlatAngled") {
                div { title: "MdAirlineSeatFlatAngled",
                    Icon {
                        icon: MdAirlineSeatFlatAngled,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdAirlineSeatIndividualSuite") {
                div { title: "MdAirlineSeatIndividualSuite",
                    Icon {
                        icon: MdAirlineSeatIndividualSuite,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdAirlineSeatLegroomExtra") {
                div { title: "MdAirlineSeatLegroomExtra",
                    Icon {
                        icon: MdAirlineSeatLegroomExtra,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdAirlineSeatLegroomNormal") {
                div { title: "MdAirlineSeatLegroomNormal",
                    Icon {
                        icon: MdAirlineSeatLegroomNormal,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdAirlineSeatLegroomReduced") {
                div { title: "MdAirlineSeatLegroomReduced",
                    Icon {
                        icon: MdAirlineSeatLegroomReduced,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdAirlineSeatReclineExtra") {
                div { title: "MdAirlineSeatReclineExtra",
                    Icon {
                        icon: MdAirlineSeatReclineExtra,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdAirlineSeatReclineNormal") {
                div { title: "MdAirlineSeatReclineNormal",
                    Icon {
                        icon: MdAirlineSeatReclineNormal,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdAirplanemodeActive") {
                div { title: "MdAirplanemodeActive",
                    Icon { icon: MdAirplanemodeActive, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAirplanemodeInactive") {
                div { title: "MdAirplanemodeInactive",
                    Icon { icon: MdAirplanemodeInactive, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAirplay") {
                div { title: "MdAirplay",
                    Icon { icon: MdAirplay, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAirportShuttle") {
                div { title: "MdAirportShuttle",
                    Icon { icon: MdAirportShuttle, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotB(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdAlarm") {
                div { title: "MdAlarm",
                    Icon { icon: MdAlarm, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAlarmAdd") {
                div { title: "MdAlarmAdd",
                    Icon { icon: MdAlarmAdd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAlarmOff") {
                div { title: "MdAlarmOff",
                    Icon { icon: MdAlarmOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAlarmOn") {
                div { title: "MdAlarmOn",
                    Icon { icon: MdAlarmOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAlbum") {
                div { title: "MdAlbum",
                    Icon { icon: MdAlbum, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAllInbox") {
                div { title: "MdAllInbox",
                    Icon { icon: MdAllInbox, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAllInclusive") {
                div { title: "MdAllInclusive",
                    Icon { icon: MdAllInclusive, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAllOut") {
                div { title: "MdAllOut",
                    Icon { icon: MdAllOut, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAlternateEmail") {
                div { title: "MdAlternateEmail",
                    Icon { icon: MdAlternateEmail, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAltRoute") {
                div { title: "MdAltRoute",
                    Icon { icon: MdAltRoute, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAmpStories") {
                div { title: "MdAmpStories",
                    Icon { icon: MdAmpStories, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAnalytics") {
                div { title: "MdAnalytics",
                    Icon { icon: MdAnalytics, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAnchor") {
                div { title: "MdAnchor",
                    Icon { icon: MdAnchor, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAndroid") {
                div { title: "MdAndroid",
                    Icon { icon: MdAndroid, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAnimation") {
                div { title: "MdAnimation",
                    Icon { icon: MdAnimation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAnnouncement") {
                div { title: "MdAnnouncement",
                    Icon { icon: MdAnnouncement, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdApartment") {
                div { title: "MdApartment",
                    Icon { icon: MdApartment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdApi") {
                div { title: "MdApi",
                    Icon { icon: MdApi, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAppBlocking") {
                div { title: "MdAppBlocking",
                    Icon { icon: MdAppBlocking, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAppRegistration") {
                div { title: "MdAppRegistration",
                    Icon { icon: MdAppRegistration, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdApproval") {
                div { title: "MdApproval",
                    Icon { icon: MdApproval, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdApps") {
                div { title: "MdApps",
                    Icon { icon: MdApps, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAppSettingsAlt") {
                div { title: "MdAppSettingsAlt",
                    Icon { icon: MdAppSettingsAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArchitecture") {
                div { title: "MdArchitecture",
                    Icon { icon: MdArchitecture, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArchive") {
                div { title: "MdArchive",
                    Icon { icon: MdArchive, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowBack") {
                div { title: "MdArrowBack",
                    Icon { icon: MdArrowBack, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowBackIos") {
                div { title: "MdArrowBackIos",
                    Icon { icon: MdArrowBackIos, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowCircleDown") {
                div { title: "MdArrowCircleDown",
                    Icon { icon: MdArrowCircleDown, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowCircleUp") {
                div { title: "MdArrowCircleUp",
                    Icon { icon: MdArrowCircleUp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowDownward") {
                div { title: "MdArrowDownward",
                    Icon { icon: MdArrowDownward, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowDropDown") {
                div { title: "MdArrowDropDown",
                    Icon { icon: MdArrowDropDown, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowDropDownCircle") {
                div { title: "MdArrowDropDownCircle",
                    Icon { icon: MdArrowDropDownCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowDropUp") {
                div { title: "MdArrowDropUp",
                    Icon { icon: MdArrowDropUp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowForward") {
                div { title: "MdArrowForward",
                    Icon { icon: MdArrowForward, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowForwardIos") {
                div { title: "MdArrowForwardIos",
                    Icon { icon: MdArrowForwardIos, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowLeft") {
                div { title: "MdArrowLeft",
                    Icon { icon: MdArrowLeft, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowRight") {
                div { title: "MdArrowRight",
                    Icon { icon: MdArrowRight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowRightAlt") {
                div { title: "MdArrowRightAlt",
                    Icon { icon: MdArrowRightAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArrowUpward") {
                div { title: "MdArrowUpward",
                    Icon { icon: MdArrowUpward, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArticle") {
                div { title: "MdArticle",
                    Icon { icon: MdArticle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdArtTrack") {
                div { title: "MdArtTrack",
                    Icon { icon: MdArtTrack, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAspectRatio") {
                div { title: "MdAspectRatio",
                    Icon { icon: MdAspectRatio, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssessment") {
                div { title: "MdAssessment",
                    Icon { icon: MdAssessment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssignment") {
                div { title: "MdAssignment",
                    Icon { icon: MdAssignment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssignmentInd") {
                div { title: "MdAssignmentInd",
                    Icon { icon: MdAssignmentInd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssignmentLate") {
                div { title: "MdAssignmentLate",
                    Icon { icon: MdAssignmentLate, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssignmentReturn") {
                div { title: "MdAssignmentReturn",
                    Icon { icon: MdAssignmentReturn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssignmentReturned") {
                div { title: "MdAssignmentReturned",
                    Icon { icon: MdAssignmentReturned, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssignmentTurnedIn") {
                div { title: "MdAssignmentTurnedIn",
                    Icon { icon: MdAssignmentTurnedIn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssistant") {
                div { title: "MdAssistant",
                    Icon { icon: MdAssistant, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssistantDirection") {
                div { title: "MdAssistantDirection",
                    Icon { icon: MdAssistantDirection, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssistantNavigation") {
                div { title: "MdAssistantNavigation",
                    Icon { icon: MdAssistantNavigation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAssistantPhoto") {
                div { title: "MdAssistantPhoto",
                    Icon { icon: MdAssistantPhoto, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAtm") {
                div { title: "MdAtm",
                    Icon { icon: MdAtm, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAttachEmail") {
                div { title: "MdAttachEmail",
                    Icon { icon: MdAttachEmail, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAttachFile") {
                div { title: "MdAttachFile",
                    Icon { icon: MdAttachFile, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAttachment") {
                div { title: "MdAttachment",
                    Icon { icon: MdAttachment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAttachMoney") {
                div { title: "MdAttachMoney",
                    Icon { icon: MdAttachMoney, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAttractions") {
                div { title: "MdAttractions",
                    Icon { icon: MdAttractions, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAudiotrack") {
                div { title: "MdAudiotrack",
                    Icon { icon: MdAudiotrack, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAutoAwesome") {
                div { title: "MdAutoAwesome",
                    Icon { icon: MdAutoAwesome, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAutoAwesomeMosaic") {
                div { title: "MdAutoAwesomeMosaic",
                    Icon { icon: MdAutoAwesomeMosaic, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAutoAwesomeMotion") {
                div { title: "MdAutoAwesomeMotion",
                    Icon { icon: MdAutoAwesomeMotion, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAutoDelete") {
                div { title: "MdAutoDelete",
                    Icon { icon: MdAutoDelete, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAutoFixHigh") {
                div { title: "MdAutoFixHigh",
                    Icon { icon: MdAutoFixHigh, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAutoFixNormal") {
                div { title: "MdAutoFixNormal",
                    Icon { icon: MdAutoFixNormal, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAutoFixOff") {
                div { title: "MdAutoFixOff",
                    Icon { icon: MdAutoFixOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAutorenew") {
                div { title: "MdAutorenew",
                    Icon { icon: MdAutorenew, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAutoStories") {
                div { title: "MdAutoStories",
                    Icon { icon: MdAutoStories, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdAvTimer") {
                div { title: "MdAvTimer",
                    Icon { icon: MdAvTimer, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBabyChangingStation") {
                div { title: "MdBabyChangingStation",
                    Icon { icon: MdBabyChangingStation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBackpack") {
                div { title: "MdBackpack",
                    Icon { icon: MdBackpack, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBackspace") {
                div { title: "MdBackspace",
                    Icon { icon: MdBackspace, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBackup") {
                div { title: "MdBackup",
                    Icon { icon: MdBackup, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBackupTable") {
                div { title: "MdBackupTable",
                    Icon { icon: MdBackupTable, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBadge") {
                div { title: "MdBadge",
                    Icon { icon: MdBadge, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBakeryDining") {
                div { title: "MdBakeryDining",
                    Icon { icon: MdBakeryDining, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBallot") {
                div { title: "MdBallot",
                    Icon { icon: MdBallot, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBarChart") {
                div { title: "MdBarChart",
                    Icon { icon: MdBarChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBatchPrediction") {
                div { title: "MdBatchPrediction",
                    Icon { icon: MdBatchPrediction, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBathtub") {
                div { title: "MdBathtub",
                    Icon { icon: MdBathtub, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBatteryAlert") {
                div { title: "MdBatteryAlert",
                    Icon { icon: MdBatteryAlert, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBatteryChargingFull") {
                div { title: "MdBatteryChargingFull",
                    Icon { icon: MdBatteryChargingFull, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBatteryFull") {
                div { title: "MdBatteryFull",
                    Icon { icon: MdBatteryFull, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBatteryStd") {
                div { title: "MdBatteryStd",
                    Icon { icon: MdBatteryStd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBatteryUnknown") {
                div { title: "MdBatteryUnknown",
                    Icon { icon: MdBatteryUnknown, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBeachAccess") {
                div { title: "MdBeachAccess",
                    Icon { icon: MdBeachAccess, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBedtime") {
                div { title: "MdBedtime",
                    Icon { icon: MdBedtime, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBeenhere") {
                div { title: "MdBeenhere",
                    Icon { icon: MdBeenhere, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBento") {
                div { title: "MdBento",
                    Icon { icon: MdBento, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBikeScooter") {
                div { title: "MdBikeScooter",
                    Icon { icon: MdBikeScooter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBiotech") {
                div { title: "MdBiotech",
                    Icon { icon: MdBiotech, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBlock") {
                div { title: "MdBlock",
                    Icon { icon: MdBlock, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBlockFlipped") {
                div { title: "MdBlockFlipped",
                    Icon { icon: MdBlockFlipped, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBluetooth") {
                div { title: "MdBluetooth",
                    Icon { icon: MdBluetooth, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBluetoothAudio") {
                div { title: "MdBluetoothAudio",
                    Icon { icon: MdBluetoothAudio, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBluetoothConnected") {
                div { title: "MdBluetoothConnected",
                    Icon { icon: MdBluetoothConnected, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBluetoothDisabled") {
                div { title: "MdBluetoothDisabled",
                    Icon { icon: MdBluetoothDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBluetoothSearching") {
                div { title: "MdBluetoothSearching",
                    Icon { icon: MdBluetoothSearching, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBlurCircular") {
                div { title: "MdBlurCircular",
                    Icon { icon: MdBlurCircular, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotC(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdBlurLinear") {
                div { title: "MdBlurLinear",
                    Icon { icon: MdBlurLinear, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBlurOff") {
                div { title: "MdBlurOff",
                    Icon { icon: MdBlurOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBlurOn") {
                div { title: "MdBlurOn",
                    Icon { icon: MdBlurOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBolt") {
                div { title: "MdBolt",
                    Icon { icon: MdBolt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBook") {
                div { title: "MdBook",
                    Icon { icon: MdBook, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBookmark") {
                div { title: "MdBookmark",
                    Icon { icon: MdBookmark, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBookmarkBorder") {
                div { title: "MdBookmarkBorder",
                    Icon { icon: MdBookmarkBorder, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBookmarks") {
                div { title: "MdBookmarks",
                    Icon { icon: MdBookmarks, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBookOnline") {
                div { title: "MdBookOnline",
                    Icon { icon: MdBookOnline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderAll") {
                div { title: "MdBorderAll",
                    Icon { icon: MdBorderAll, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderBottom") {
                div { title: "MdBorderBottom",
                    Icon { icon: MdBorderBottom, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderClear") {
                div { title: "MdBorderClear",
                    Icon { icon: MdBorderClear, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderColor") {
                div { title: "MdBorderColor",
                    Icon { icon: MdBorderColor, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderHorizontal") {
                div { title: "MdBorderHorizontal",
                    Icon { icon: MdBorderHorizontal, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderInner") {
                div { title: "MdBorderInner",
                    Icon { icon: MdBorderInner, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderLeft") {
                div { title: "MdBorderLeft",
                    Icon { icon: MdBorderLeft, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderOuter") {
                div { title: "MdBorderOuter",
                    Icon { icon: MdBorderOuter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderRight") {
                div { title: "MdBorderRight",
                    Icon { icon: MdBorderRight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderStyle") {
                div { title: "MdBorderStyle",
                    Icon { icon: MdBorderStyle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderTop") {
                div { title: "MdBorderTop",
                    Icon { icon: MdBorderTop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBorderVertical") {
                div { title: "MdBorderVertical",
                    Icon { icon: MdBorderVertical, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrandingWatermark") {
                div { title: "MdBrandingWatermark",
                    Icon { icon: MdBrandingWatermark, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBreakfastDining") {
                div { title: "MdBreakfastDining",
                    Icon { icon: MdBreakfastDining, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightness1") {
                div { title: "MdBrightness1",
                    Icon { icon: MdBrightness1, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightness2") {
                div { title: "MdBrightness2",
                    Icon { icon: MdBrightness2, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightness3") {
                div { title: "MdBrightness3",
                    Icon { icon: MdBrightness3, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightness4") {
                div { title: "MdBrightness4",
                    Icon { icon: MdBrightness4, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightness5") {
                div { title: "MdBrightness5",
                    Icon { icon: MdBrightness5, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightness6") {
                div { title: "MdBrightness6",
                    Icon { icon: MdBrightness6, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightness7") {
                div { title: "MdBrightness7",
                    Icon { icon: MdBrightness7, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightnessAuto") {
                div { title: "MdBrightnessAuto",
                    Icon { icon: MdBrightnessAuto, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightnessHigh") {
                div { title: "MdBrightnessHigh",
                    Icon { icon: MdBrightnessHigh, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightnessLow") {
                div { title: "MdBrightnessLow",
                    Icon { icon: MdBrightnessLow, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrightnessMedium") {
                div { title: "MdBrightnessMedium",
                    Icon { icon: MdBrightnessMedium, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrokenImage") {
                div { title: "MdBrokenImage",
                    Icon { icon: MdBrokenImage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrowserNotSupported") {
                div { title: "MdBrowserNotSupported",
                    Icon { icon: MdBrowserNotSupported, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrunchDining") {
                div { title: "MdBrunchDining",
                    Icon { icon: MdBrunchDining, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBrush") {
                div { title: "MdBrush",
                    Icon { icon: MdBrush, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBubbleChart") {
                div { title: "MdBubbleChart",
                    Icon { icon: MdBubbleChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBugReport") {
                div { title: "MdBugReport",
                    Icon { icon: MdBugReport, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBuild") {
                div { title: "MdBuild",
                    Icon { icon: MdBuild, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBuildCircle") {
                div { title: "MdBuildCircle",
                    Icon { icon: MdBuildCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBurstMode") {
                div { title: "MdBurstMode",
                    Icon { icon: MdBurstMode, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBusAlert") {
                div { title: "MdBusAlert",
                    Icon { icon: MdBusAlert, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBusiness") {
                div { title: "MdBusiness",
                    Icon { icon: MdBusiness, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdBusinessCenter") {
                div { title: "MdBusinessCenter",
                    Icon { icon: MdBusinessCenter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCached") {
                div { title: "MdCached",
                    Icon { icon: MdCached, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCake") {
                div { title: "MdCake",
                    Icon { icon: MdCake, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCalculate") {
                div { title: "MdCalculate",
                    Icon { icon: MdCalculate, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCalendarToday") {
                div { title: "MdCalendarToday",
                    Icon { icon: MdCalendarToday, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCalendarViewDay") {
                div { title: "MdCalendarViewDay",
                    Icon { icon: MdCalendarViewDay, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCall") {
                div { title: "MdCall",
                    Icon { icon: MdCall, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCallEnd") {
                div { title: "MdCallEnd",
                    Icon { icon: MdCallEnd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCallMade") {
                div { title: "MdCallMade",
                    Icon { icon: MdCallMade, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCallMerge") {
                div { title: "MdCallMerge",
                    Icon { icon: MdCallMerge, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCallMissed") {
                div { title: "MdCallMissed",
                    Icon { icon: MdCallMissed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCallMissedOutgoing") {
                div { title: "MdCallMissedOutgoing",
                    Icon { icon: MdCallMissedOutgoing, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCallReceived") {
                div { title: "MdCallReceived",
                    Icon { icon: MdCallReceived, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCallSplit") {
                div { title: "MdCallSplit",
                    Icon { icon: MdCallSplit, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCallToAction") {
                div { title: "MdCallToAction",
                    Icon { icon: MdCallToAction, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCamera") {
                div { title: "MdCamera",
                    Icon { icon: MdCamera, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCameraAlt") {
                div { title: "MdCameraAlt",
                    Icon { icon: MdCameraAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCameraEnhance") {
                div { title: "MdCameraEnhance",
                    Icon { icon: MdCameraEnhance, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCameraFront") {
                div { title: "MdCameraFront",
                    Icon { icon: MdCameraFront, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCameraRear") {
                div { title: "MdCameraRear",
                    Icon { icon: MdCameraRear, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCameraRoll") {
                div { title: "MdCameraRoll",
                    Icon { icon: MdCameraRoll, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCampaign") {
                div { title: "MdCampaign",
                    Icon { icon: MdCampaign, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCancel") {
                div { title: "MdCancel",
                    Icon { icon: MdCancel, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCancelPresentation") {
                div { title: "MdCancelPresentation",
                    Icon { icon: MdCancelPresentation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCancelScheduleSend") {
                div { title: "MdCancelScheduleSend",
                    Icon { icon: MdCancelScheduleSend, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCardGiftcard") {
                div { title: "MdCardGiftcard",
                    Icon { icon: MdCardGiftcard, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCardMembership") {
                div { title: "MdCardMembership",
                    Icon { icon: MdCardMembership, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCardTravel") {
                div { title: "MdCardTravel",
                    Icon { icon: MdCardTravel, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCarpenter") {
                div { title: "MdCarpenter",
                    Icon { icon: MdCarpenter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCarRental") {
                div { title: "MdCarRental",
                    Icon { icon: MdCarRental, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCarRepair") {
                div { title: "MdCarRepair",
                    Icon { icon: MdCarRepair, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCases") {
                div { title: "MdCases",
                    Icon { icon: MdCases, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCasino") {
                div { title: "MdCasino",
                    Icon { icon: MdCasino, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCast") {
                div { title: "MdCast",
                    Icon { icon: MdCast, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCastConnected") {
                div { title: "MdCastConnected",
                    Icon { icon: MdCastConnected, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCastForEducation") {
                div { title: "MdCastForEducation",
                    Icon { icon: MdCastForEducation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCategory") {
                div { title: "MdCategory",
                    Icon { icon: MdCategory, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCelebration") {
                div { title: "MdCelebration",
                    Icon { icon: MdCelebration, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCellWifi") {
                div { title: "MdCellWifi",
                    Icon { icon: MdCellWifi, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCenterFocusStrong") {
                div { title: "MdCenterFocusStrong",
                    Icon { icon: MdCenterFocusStrong, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCenterFocusWeak") {
                div { title: "MdCenterFocusWeak",
                    Icon { icon: MdCenterFocusWeak, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdChangeHistory") {
                div { title: "MdChangeHistory",
                    Icon { icon: MdChangeHistory, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdChargingStation") {
                div { title: "MdChargingStation",
                    Icon { icon: MdChargingStation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdChat") {
                div { title: "MdChat",
                    Icon { icon: MdChat, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdChatBubble") {
                div { title: "MdChatBubble",
                    Icon { icon: MdChatBubble, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdChatBubbleOutline") {
                div { title: "MdChatBubbleOutline",
                    Icon { icon: MdChatBubbleOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCheck") {
                div { title: "MdCheck",
                    Icon { icon: MdCheck, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCheckBox") {
                div { title: "MdCheckBox",
                    Icon { icon: MdCheckBox, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCheckBoxOutlineBlank") {
                div { title: "MdCheckBoxOutlineBlank",
                    Icon { icon: MdCheckBoxOutlineBlank, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCheckCircle") {
                div { title: "MdCheckCircle",
                    Icon { icon: MdCheckCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCheckCircleOutline") {
                div { title: "MdCheckCircleOutline",
                    Icon { icon: MdCheckCircleOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCheckroom") {
                div { title: "MdCheckroom",
                    Icon { icon: MdCheckroom, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdChevronLeft") {
                div { title: "MdChevronLeft",
                    Icon { icon: MdChevronLeft, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdChevronRight") {
                div { title: "MdChevronRight",
                    Icon { icon: MdChevronRight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdChildCare") {
                div { title: "MdChildCare",
                    Icon { icon: MdChildCare, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotD(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdChildFriendly") {
                div { title: "MdChildFriendly",
                    Icon { icon: MdChildFriendly, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdChromeReaderMode") {
                div { title: "MdChromeReaderMode",
                    Icon { icon: MdChromeReaderMode, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCircle") {
                div { title: "MdCircle",
                    Icon { icon: MdCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCircleNotifications") {
                div { title: "MdCircleNotifications",
                    Icon { icon: MdCircleNotifications, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdClass") {
                div { title: "MdClass",
                    Icon { icon: MdClass, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCleanHands") {
                div { title: "MdCleanHands",
                    Icon { icon: MdCleanHands, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCleaningServices") {
                div { title: "MdCleaningServices",
                    Icon { icon: MdCleaningServices, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdClear") {
                div { title: "MdClear",
                    Icon { icon: MdClear, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdClearAll") {
                div { title: "MdClearAll",
                    Icon { icon: MdClearAll, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdClose") {
                div { title: "MdClose",
                    Icon { icon: MdClose, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdClosedCaption") {
                div { title: "MdClosedCaption",
                    Icon { icon: MdClosedCaption, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdClosedCaptionDisabled") {
                div { title: "MdClosedCaptionDisabled",
                    Icon {
                        icon: MdClosedCaptionDisabled,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdClosedCaptionOff") {
                div { title: "MdClosedCaptionOff",
                    Icon { icon: MdClosedCaptionOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCloseFullscreen") {
                div { title: "MdCloseFullscreen",
                    Icon { icon: MdCloseFullscreen, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCloud") {
                div { title: "MdCloud",
                    Icon { icon: MdCloud, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCloudCircle") {
                div { title: "MdCloudCircle",
                    Icon { icon: MdCloudCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCloudDone") {
                div { title: "MdCloudDone",
                    Icon { icon: MdCloudDone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCloudDownload") {
                div { title: "MdCloudDownload",
                    Icon { icon: MdCloudDownload, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCloudOff") {
                div { title: "MdCloudOff",
                    Icon { icon: MdCloudOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCloudQueue") {
                div { title: "MdCloudQueue",
                    Icon { icon: MdCloudQueue, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCloudUpload") {
                div { title: "MdCloudUpload",
                    Icon { icon: MdCloudUpload, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCode") {
                div { title: "MdCode",
                    Icon { icon: MdCode, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCollections") {
                div { title: "MdCollections",
                    Icon { icon: MdCollections, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCollectionsBookmark") {
                div { title: "MdCollectionsBookmark",
                    Icon { icon: MdCollectionsBookmark, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdColorize") {
                div { title: "MdColorize",
                    Icon { icon: MdColorize, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdColorLens") {
                div { title: "MdColorLens",
                    Icon { icon: MdColorLens, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdComment") {
                div { title: "MdComment",
                    Icon { icon: MdComment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCommentBank") {
                div { title: "MdCommentBank",
                    Icon { icon: MdCommentBank, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCommute") {
                div { title: "MdCommute",
                    Icon { icon: MdCommute, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCompare") {
                div { title: "MdCompare",
                    Icon { icon: MdCompare, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCompareArrows") {
                div { title: "MdCompareArrows",
                    Icon { icon: MdCompareArrows, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCompassCalibration") {
                div { title: "MdCompassCalibration",
                    Icon { icon: MdCompassCalibration, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCompress") {
                div { title: "MdCompress",
                    Icon { icon: MdCompress, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdComputer") {
                div { title: "MdComputer",
                    Icon { icon: MdComputer, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdConfirmationNumber") {
                div { title: "MdConfirmationNumber",
                    Icon { icon: MdConfirmationNumber, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdConnectedTv") {
                div { title: "MdConnectedTv",
                    Icon { icon: MdConnectedTv, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdConnectWithoutContact") {
                div { title: "MdConnectWithoutContact",
                    Icon {
                        icon: MdConnectWithoutContact,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdConstruction") {
                div { title: "MdConstruction",
                    Icon { icon: MdConstruction, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdContactless") {
                div { title: "MdContactless",
                    Icon { icon: MdContactless, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdContactMail") {
                div { title: "MdContactMail",
                    Icon { icon: MdContactMail, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdContactPage") {
                div { title: "MdContactPage",
                    Icon { icon: MdContactPage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdContactPhone") {
                div { title: "MdContactPhone",
                    Icon { icon: MdContactPhone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdContacts") {
                div { title: "MdContacts",
                    Icon { icon: MdContacts, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdContactSupport") {
                div { title: "MdContactSupport",
                    Icon { icon: MdContactSupport, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdContentCopy") {
                div { title: "MdContentCopy",
                    Icon { icon: MdContentCopy, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdContentCut") {
                div { title: "MdContentCut",
                    Icon { icon: MdContentCut, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdContentPaste") {
                div { title: "MdContentPaste",
                    Icon { icon: MdContentPaste, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdControlCamera") {
                div { title: "MdControlCamera",
                    Icon { icon: MdControlCamera, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdControlPoint") {
                div { title: "MdControlPoint",
                    Icon { icon: MdControlPoint, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdControlPointDuplicate") {
                div { title: "MdControlPointDuplicate",
                    Icon {
                        icon: MdControlPointDuplicate,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdCopyright") {
                div { title: "MdCopyright",
                    Icon { icon: MdCopyright, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCoronavirus") {
                div { title: "MdCoronavirus",
                    Icon { icon: MdCoronavirus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCorporateFare") {
                div { title: "MdCorporateFare",
                    Icon { icon: MdCorporateFare, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCountertops") {
                div { title: "MdCountertops",
                    Icon { icon: MdCountertops, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCreate") {
                div { title: "MdCreate",
                    Icon { icon: MdCreate, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCreateNewFolder") {
                div { title: "MdCreateNewFolder",
                    Icon { icon: MdCreateNewFolder, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCreditCard") {
                div { title: "MdCreditCard",
                    Icon { icon: MdCreditCard, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCrop") {
                div { title: "MdCrop",
                    Icon { icon: MdCrop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCrop32") {
                div { title: "MdCrop32",
                    Icon { icon: MdCrop32, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCrop54") {
                div { title: "MdCrop54",
                    Icon { icon: MdCrop54, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCrop75") {
                div { title: "MdCrop75",
                    Icon { icon: MdCrop75, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCrop169") {
                div { title: "MdCrop169",
                    Icon { icon: MdCrop169, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCropDin") {
                div { title: "MdCropDin",
                    Icon { icon: MdCropDin, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCropFree") {
                div { title: "MdCropFree",
                    Icon { icon: MdCropFree, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCropLandscape") {
                div { title: "MdCropLandscape",
                    Icon { icon: MdCropLandscape, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCropOriginal") {
                div { title: "MdCropOriginal",
                    Icon { icon: MdCropOriginal, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCropPortrait") {
                div { title: "MdCropPortrait",
                    Icon { icon: MdCropPortrait, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCropRotate") {
                div { title: "MdCropRotate",
                    Icon { icon: MdCropRotate, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdCropSquare") {
                div { title: "MdCropSquare",
                    Icon { icon: MdCropSquare, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDangerous") {
                div { title: "MdDangerous",
                    Icon { icon: MdDangerous, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDashboard") {
                div { title: "MdDashboard",
                    Icon { icon: MdDashboard, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDashboardCustomize") {
                div { title: "MdDashboardCustomize",
                    Icon { icon: MdDashboardCustomize, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDataUsage") {
                div { title: "MdDataUsage",
                    Icon { icon: MdDataUsage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDateRange") {
                div { title: "MdDateRange",
                    Icon { icon: MdDateRange, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDeck") {
                div { title: "MdDeck",
                    Icon { icon: MdDeck, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDehaze") {
                div { title: "MdDehaze",
                    Icon { icon: MdDehaze, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDelete") {
                div { title: "MdDelete",
                    Icon { icon: MdDelete, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDeleteForever") {
                div { title: "MdDeleteForever",
                    Icon { icon: MdDeleteForever, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDeleteOutline") {
                div { title: "MdDeleteOutline",
                    Icon { icon: MdDeleteOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDeleteSweep") {
                div { title: "MdDeleteSweep",
                    Icon { icon: MdDeleteSweep, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDeliveryDining") {
                div { title: "MdDeliveryDining",
                    Icon { icon: MdDeliveryDining, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDepartureBoard") {
                div { title: "MdDepartureBoard",
                    Icon { icon: MdDepartureBoard, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDescription") {
                div { title: "MdDescription",
                    Icon { icon: MdDescription, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDesignServices") {
                div { title: "MdDesignServices",
                    Icon { icon: MdDesignServices, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDesktopAccessDisabled") {
                div { title: "MdDesktopAccessDisabled",
                    Icon {
                        icon: MdDesktopAccessDisabled,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdDesktopMac") {
                div { title: "MdDesktopMac",
                    Icon { icon: MdDesktopMac, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDesktopWindows") {
                div { title: "MdDesktopWindows",
                    Icon { icon: MdDesktopWindows, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDetails") {
                div { title: "MdDetails",
                    Icon { icon: MdDetails, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDeveloperBoard") {
                div { title: "MdDeveloperBoard",
                    Icon { icon: MdDeveloperBoard, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDeveloperMode") {
                div { title: "MdDeveloperMode",
                    Icon { icon: MdDeveloperMode, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDeviceHub") {
                div { title: "MdDeviceHub",
                    Icon { icon: MdDeviceHub, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDevices") {
                div { title: "MdDevices",
                    Icon { icon: MdDevices, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDevicesOther") {
                div { title: "MdDevicesOther",
                    Icon { icon: MdDevicesOther, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDeviceThermostat") {
                div { title: "MdDeviceThermostat",
                    Icon { icon: MdDeviceThermostat, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDeviceUnknown") {
                div { title: "MdDeviceUnknown",
                    Icon { icon: MdDeviceUnknown, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDialerSip") {
                div { title: "MdDialerSip",
                    Icon { icon: MdDialerSip, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDialpad") {
                div { title: "MdDialpad",
                    Icon { icon: MdDialpad, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDinnerDining") {
                div { title: "MdDinnerDining",
                    Icon { icon: MdDinnerDining, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirections") {
                div { title: "MdDirections",
                    Icon { icon: MdDirections, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirectionsBike") {
                div { title: "MdDirectionsBike",
                    Icon { icon: MdDirectionsBike, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotE(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdDirectionsBoat") {
                div { title: "MdDirectionsBoat",
                    Icon { icon: MdDirectionsBoat, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirectionsBus") {
                div { title: "MdDirectionsBus",
                    Icon { icon: MdDirectionsBus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirectionsCar") {
                div { title: "MdDirectionsCar",
                    Icon { icon: MdDirectionsCar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirectionsOff") {
                div { title: "MdDirectionsOff",
                    Icon { icon: MdDirectionsOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirectionsRailway") {
                div { title: "MdDirectionsRailway",
                    Icon { icon: MdDirectionsRailway, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirectionsRun") {
                div { title: "MdDirectionsRun",
                    Icon { icon: MdDirectionsRun, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirectionsSubway") {
                div { title: "MdDirectionsSubway",
                    Icon { icon: MdDirectionsSubway, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirectionsTransit") {
                div { title: "MdDirectionsTransit",
                    Icon { icon: MdDirectionsTransit, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirectionsWalk") {
                div { title: "MdDirectionsWalk",
                    Icon { icon: MdDirectionsWalk, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDirtyLens") {
                div { title: "MdDirtyLens",
                    Icon { icon: MdDirtyLens, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDisabledByDefault") {
                div { title: "MdDisabledByDefault",
                    Icon { icon: MdDisabledByDefault, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDiscFull") {
                div { title: "MdDiscFull",
                    Icon { icon: MdDiscFull, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDns") {
                div { title: "MdDns",
                    Icon { icon: MdDns, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDock") {
                div { title: "MdDock",
                    Icon { icon: MdDock, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDomain") {
                div { title: "MdDomain",
                    Icon { icon: MdDomain, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDomainDisabled") {
                div { title: "MdDomainDisabled",
                    Icon { icon: MdDomainDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDomainVerification") {
                div { title: "MdDomainVerification",
                    Icon { icon: MdDomainVerification, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDone") {
                div { title: "MdDone",
                    Icon { icon: MdDone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDoneAll") {
                div { title: "MdDoneAll",
                    Icon { icon: MdDoneAll, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDoneOutline") {
                div { title: "MdDoneOutline",
                    Icon { icon: MdDoneOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDoNotDisturb") {
                div { title: "MdDoNotDisturb",
                    Icon { icon: MdDoNotDisturb, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDoNotDisturbAlt") {
                div { title: "MdDoNotDisturbAlt",
                    Icon { icon: MdDoNotDisturbAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDoNotDisturbOff") {
                div { title: "MdDoNotDisturbOff",
                    Icon { icon: MdDoNotDisturbOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDoNotDisturbOn") {
                div { title: "MdDoNotDisturbOn",
                    Icon { icon: MdDoNotDisturbOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDoNotStep") {
                div { title: "MdDoNotStep",
                    Icon { icon: MdDoNotStep, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDoNotTouch") {
                div { title: "MdDoNotTouch",
                    Icon { icon: MdDoNotTouch, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDonutLarge") {
                div { title: "MdDonutLarge",
                    Icon { icon: MdDonutLarge, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDonutSmall") {
                div { title: "MdDonutSmall",
                    Icon { icon: MdDonutSmall, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDoubleArrow") {
                div { title: "MdDoubleArrow",
                    Icon { icon: MdDoubleArrow, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDrafts") {
                div { title: "MdDrafts",
                    Icon { icon: MdDrafts, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDragHandle") {
                div { title: "MdDragHandle",
                    Icon { icon: MdDragHandle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDragIndicator") {
                div { title: "MdDragIndicator",
                    Icon { icon: MdDragIndicator, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDriveEta") {
                div { title: "MdDriveEta",
                    Icon { icon: MdDriveEta, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDriveFileMove") {
                div { title: "MdDriveFileMove",
                    Icon { icon: MdDriveFileMove, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDriveFileMoveOutline") {
                div { title: "MdDriveFileMoveOutline",
                    Icon { icon: MdDriveFileMoveOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDriveFileRenameOutline") {
                div { title: "MdDriveFileRenameOutline",
                    Icon {
                        icon: MdDriveFileRenameOutline,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdDriveFolderUpload") {
                div { title: "MdDriveFolderUpload",
                    Icon { icon: MdDriveFolderUpload, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDry") {
                div { title: "MdDry",
                    Icon { icon: MdDry, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDryCleaning") {
                div { title: "MdDryCleaning",
                    Icon { icon: MdDryCleaning, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDuo") {
                div { title: "MdDuo",
                    Icon { icon: MdDuo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDvr") {
                div { title: "MdDvr",
                    Icon { icon: MdDvr, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDynamicFeed") {
                div { title: "MdDynamicFeed",
                    Icon { icon: MdDynamicFeed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdDynamicForm") {
                div { title: "MdDynamicForm",
                    Icon { icon: MdDynamicForm, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEast") {
                div { title: "MdEast",
                    Icon { icon: MdEast, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEco") {
                div { title: "MdEco",
                    Icon { icon: MdEco, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEdit") {
                div { title: "MdEdit",
                    Icon { icon: MdEdit, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEditAttributes") {
                div { title: "MdEditAttributes",
                    Icon { icon: MdEditAttributes, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEditLocation") {
                div { title: "MdEditLocation",
                    Icon { icon: MdEditLocation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEditOff") {
                div { title: "MdEditOff",
                    Icon { icon: MdEditOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEditRoad") {
                div { title: "MdEditRoad",
                    Icon { icon: MdEditRoad, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEject") {
                div { title: "MdEject",
                    Icon { icon: MdEject, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdElderly") {
                div { title: "MdElderly",
                    Icon { icon: MdElderly, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdElectricalServices") {
                div { title: "MdElectricalServices",
                    Icon { icon: MdElectricalServices, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdElectricBike") {
                div { title: "MdElectricBike",
                    Icon { icon: MdElectricBike, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdElectricCar") {
                div { title: "MdElectricCar",
                    Icon { icon: MdElectricCar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdElectricMoped") {
                div { title: "MdElectricMoped",
                    Icon { icon: MdElectricMoped, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdElectricRickshaw") {
                div { title: "MdElectricRickshaw",
                    Icon { icon: MdElectricRickshaw, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdElectricScooter") {
                div { title: "MdElectricScooter",
                    Icon { icon: MdElectricScooter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdElevator") {
                div { title: "MdElevator",
                    Icon { icon: MdElevator, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEmail") {
                div { title: "MdEmail",
                    Icon { icon: MdEmail, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEmojiEmotions") {
                div { title: "MdEmojiEmotions",
                    Icon { icon: MdEmojiEmotions, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEmojiEvents") {
                div { title: "MdEmojiEvents",
                    Icon { icon: MdEmojiEvents, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEmojiFlags") {
                div { title: "MdEmojiFlags",
                    Icon { icon: MdEmojiFlags, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEmojiFoodBeverage") {
                div { title: "MdEmojiFoodBeverage",
                    Icon { icon: MdEmojiFoodBeverage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEmojiNature") {
                div { title: "MdEmojiNature",
                    Icon { icon: MdEmojiNature, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEmojiObjects") {
                div { title: "MdEmojiObjects",
                    Icon { icon: MdEmojiObjects, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEmojiPeople") {
                div { title: "MdEmojiPeople",
                    Icon { icon: MdEmojiPeople, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEmojiSymbols") {
                div { title: "MdEmojiSymbols",
                    Icon { icon: MdEmojiSymbols, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEmojiTransportation") {
                div { title: "MdEmojiTransportation",
                    Icon { icon: MdEmojiTransportation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEngineering") {
                div { title: "MdEngineering",
                    Icon { icon: MdEngineering, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEnhancedEncryption") {
                div { title: "MdEnhancedEncryption",
                    Icon { icon: MdEnhancedEncryption, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEqualizer") {
                div { title: "MdEqualizer",
                    Icon { icon: MdEqualizer, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdError") {
                div { title: "MdError",
                    Icon { icon: MdError, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdErrorOutline") {
                div { title: "MdErrorOutline",
                    Icon { icon: MdErrorOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEscalator") {
                div { title: "MdEscalator",
                    Icon { icon: MdEscalator, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEscalatorWarning") {
                div { title: "MdEscalatorWarning",
                    Icon { icon: MdEscalatorWarning, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEuro") {
                div { title: "MdEuro",
                    Icon { icon: MdEuro, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEuroSymbol") {
                div { title: "MdEuroSymbol",
                    Icon { icon: MdEuroSymbol, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEvent") {
                div { title: "MdEvent",
                    Icon { icon: MdEvent, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEventAvailable") {
                div { title: "MdEventAvailable",
                    Icon { icon: MdEventAvailable, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEventBusy") {
                div { title: "MdEventBusy",
                    Icon { icon: MdEventBusy, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEventNote") {
                div { title: "MdEventNote",
                    Icon { icon: MdEventNote, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEventSeat") {
                div { title: "MdEventSeat",
                    Icon { icon: MdEventSeat, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdEvStation") {
                div { title: "MdEvStation",
                    Icon { icon: MdEvStation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExitToApp") {
                div { title: "MdExitToApp",
                    Icon { icon: MdExitToApp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExpand") {
                div { title: "MdExpand",
                    Icon { icon: MdExpand, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExpandLess") {
                div { title: "MdExpandLess",
                    Icon { icon: MdExpandLess, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExpandMore") {
                div { title: "MdExpandMore",
                    Icon { icon: MdExpandMore, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExplicit") {
                div { title: "MdExplicit",
                    Icon { icon: MdExplicit, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExplore") {
                div { title: "MdExplore",
                    Icon { icon: MdExplore, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExploreOff") {
                div { title: "MdExploreOff",
                    Icon { icon: MdExploreOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExposure") {
                div { title: "MdExposure",
                    Icon { icon: MdExposure, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExposureNeg1") {
                div { title: "MdExposureNeg1",
                    Icon { icon: MdExposureNeg1, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExposureNeg2") {
                div { title: "MdExposureNeg2",
                    Icon { icon: MdExposureNeg2, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExposurePlus1") {
                div { title: "MdExposurePlus1",
                    Icon { icon: MdExposurePlus1, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExposurePlus2") {
                div { title: "MdExposurePlus2",
                    Icon { icon: MdExposurePlus2, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExposureZero") {
                div { title: "MdExposureZero",
                    Icon { icon: MdExposureZero, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdExtension") {
                div { title: "MdExtension",
                    Icon { icon: MdExtension, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFace") {
                div { title: "MdFace",
                    Icon { icon: MdFace, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFacebook") {
                div { title: "MdFacebook",
                    Icon { icon: MdFacebook, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotF(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdFaceRetouchingNatural") {
                div { title: "MdFaceRetouchingNatural",
                    Icon {
                        icon: MdFaceRetouchingNatural,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdFactCheck") {
                div { title: "MdFactCheck",
                    Icon { icon: MdFactCheck, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFamilyRestroom") {
                div { title: "MdFamilyRestroom",
                    Icon { icon: MdFamilyRestroom, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFastfood") {
                div { title: "MdFastfood",
                    Icon { icon: MdFastfood, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFastForward") {
                div { title: "MdFastForward",
                    Icon { icon: MdFastForward, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFastRewind") {
                div { title: "MdFastRewind",
                    Icon { icon: MdFastRewind, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFavorite") {
                div { title: "MdFavorite",
                    Icon { icon: MdFavorite, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFavoriteBorder") {
                div { title: "MdFavoriteBorder",
                    Icon { icon: MdFavoriteBorder, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFeaturedPlayList") {
                div { title: "MdFeaturedPlayList",
                    Icon { icon: MdFeaturedPlayList, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFeaturedVideo") {
                div { title: "MdFeaturedVideo",
                    Icon { icon: MdFeaturedVideo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFeedback") {
                div { title: "MdFeedback",
                    Icon { icon: MdFeedback, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFence") {
                div { title: "MdFence",
                    Icon { icon: MdFence, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFestival") {
                div { title: "MdFestival",
                    Icon { icon: MdFestival, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFiberDvr") {
                div { title: "MdFiberDvr",
                    Icon { icon: MdFiberDvr, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFiberManualRecord") {
                div { title: "MdFiberManualRecord",
                    Icon { icon: MdFiberManualRecord, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFiberNew") {
                div { title: "MdFiberNew",
                    Icon { icon: MdFiberNew, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFiberPin") {
                div { title: "MdFiberPin",
                    Icon { icon: MdFiberPin, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFiberSmartRecord") {
                div { title: "MdFiberSmartRecord",
                    Icon { icon: MdFiberSmartRecord, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFileCopy") {
                div { title: "MdFileCopy",
                    Icon { icon: MdFileCopy, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFileDownload") {
                div { title: "MdFileDownload",
                    Icon { icon: MdFileDownload, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFileDownloadDone") {
                div { title: "MdFileDownloadDone",
                    Icon { icon: MdFileDownloadDone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilePresent") {
                div { title: "MdFilePresent",
                    Icon { icon: MdFilePresent, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFileUpload") {
                div { title: "MdFileUpload",
                    Icon { icon: MdFileUpload, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter") {
                div { title: "MdFilter",
                    Icon { icon: MdFilter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter1") {
                div { title: "MdFilter1",
                    Icon { icon: MdFilter1, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter2") {
                div { title: "MdFilter2",
                    Icon { icon: MdFilter2, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter3") {
                div { title: "MdFilter3",
                    Icon { icon: MdFilter3, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter4") {
                div { title: "MdFilter4",
                    Icon { icon: MdFilter4, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter5") {
                div { title: "MdFilter5",
                    Icon { icon: MdFilter5, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter6") {
                div { title: "MdFilter6",
                    Icon { icon: MdFilter6, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter7") {
                div { title: "MdFilter7",
                    Icon { icon: MdFilter7, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter8") {
                div { title: "MdFilter8",
                    Icon { icon: MdFilter8, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter9") {
                div { title: "MdFilter9",
                    Icon { icon: MdFilter9, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilter9Plus") {
                div { title: "MdFilter9Plus",
                    Icon { icon: MdFilter9Plus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterAlt") {
                div { title: "MdFilterAlt",
                    Icon { icon: MdFilterAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterBAndW") {
                div { title: "MdFilterBAndW",
                    Icon { icon: MdFilterBAndW, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterCenterFocus") {
                div { title: "MdFilterCenterFocus",
                    Icon { icon: MdFilterCenterFocus, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterDrama") {
                div { title: "MdFilterDrama",
                    Icon { icon: MdFilterDrama, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterFrames") {
                div { title: "MdFilterFrames",
                    Icon { icon: MdFilterFrames, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterHdr") {
                div { title: "MdFilterHdr",
                    Icon { icon: MdFilterHdr, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterList") {
                div { title: "MdFilterList",
                    Icon { icon: MdFilterList, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterListAlt") {
                div { title: "MdFilterListAlt",
                    Icon { icon: MdFilterListAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterNone") {
                div { title: "MdFilterNone",
                    Icon { icon: MdFilterNone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterTiltShift") {
                div { title: "MdFilterTiltShift",
                    Icon { icon: MdFilterTiltShift, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFilterVintage") {
                div { title: "MdFilterVintage",
                    Icon { icon: MdFilterVintage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFindInPage") {
                div { title: "MdFindInPage",
                    Icon { icon: MdFindInPage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFindReplace") {
                div { title: "MdFindReplace",
                    Icon { icon: MdFindReplace, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFingerprint") {
                div { title: "MdFingerprint",
                    Icon { icon: MdFingerprint, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFireExtinguisher") {
                div { title: "MdFireExtinguisher",
                    Icon { icon: MdFireExtinguisher, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFireplace") {
                div { title: "MdFireplace",
                    Icon { icon: MdFireplace, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFirstPage") {
                div { title: "MdFirstPage",
                    Icon { icon: MdFirstPage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFitnessCenter") {
                div { title: "MdFitnessCenter",
                    Icon { icon: MdFitnessCenter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFitScreen") {
                div { title: "MdFitScreen",
                    Icon { icon: MdFitScreen, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlag") {
                div { title: "MdFlag",
                    Icon { icon: MdFlag, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlaky") {
                div { title: "MdFlaky",
                    Icon { icon: MdFlaky, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlare") {
                div { title: "MdFlare",
                    Icon { icon: MdFlare, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlashAuto") {
                div { title: "MdFlashAuto",
                    Icon { icon: MdFlashAuto, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlashOff") {
                div { title: "MdFlashOff",
                    Icon { icon: MdFlashOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlashOn") {
                div { title: "MdFlashOn",
                    Icon { icon: MdFlashOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlight") {
                div { title: "MdFlight",
                    Icon { icon: MdFlight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlightLand") {
                div { title: "MdFlightLand",
                    Icon { icon: MdFlightLand, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlightTakeoff") {
                div { title: "MdFlightTakeoff",
                    Icon { icon: MdFlightTakeoff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlip") {
                div { title: "MdFlip",
                    Icon { icon: MdFlip, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlipCameraAndroid") {
                div { title: "MdFlipCameraAndroid",
                    Icon { icon: MdFlipCameraAndroid, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlipCameraIos") {
                div { title: "MdFlipCameraIos",
                    Icon { icon: MdFlipCameraIos, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlipToBack") {
                div { title: "MdFlipToBack",
                    Icon { icon: MdFlipToBack, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFlipToFront") {
                div { title: "MdFlipToFront",
                    Icon { icon: MdFlipToFront, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFolder") {
                div { title: "MdFolder",
                    Icon { icon: MdFolder, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFolderOpen") {
                div { title: "MdFolderOpen",
                    Icon { icon: MdFolderOpen, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFolderShared") {
                div { title: "MdFolderShared",
                    Icon { icon: MdFolderShared, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFolderSpecial") {
                div { title: "MdFolderSpecial",
                    Icon { icon: MdFolderSpecial, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFollowTheSigns") {
                div { title: "MdFollowTheSigns",
                    Icon { icon: MdFollowTheSigns, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFontDownload") {
                div { title: "MdFontDownload",
                    Icon { icon: MdFontDownload, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFoodBank") {
                div { title: "MdFoodBank",
                    Icon { icon: MdFoodBank, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatAlignCenter") {
                div { title: "MdFormatAlignCenter",
                    Icon { icon: MdFormatAlignCenter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatAlignJustify") {
                div { title: "MdFormatAlignJustify",
                    Icon { icon: MdFormatAlignJustify, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatAlignLeft") {
                div { title: "MdFormatAlignLeft",
                    Icon { icon: MdFormatAlignLeft, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatAlignRight") {
                div { title: "MdFormatAlignRight",
                    Icon { icon: MdFormatAlignRight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatBold") {
                div { title: "MdFormatBold",
                    Icon { icon: MdFormatBold, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatClear") {
                div { title: "MdFormatClear",
                    Icon { icon: MdFormatClear, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatColorFill") {
                div { title: "MdFormatColorFill",
                    Icon { icon: MdFormatColorFill, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatColorReset") {
                div { title: "MdFormatColorReset",
                    Icon { icon: MdFormatColorReset, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatColorText") {
                div { title: "MdFormatColorText",
                    Icon { icon: MdFormatColorText, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatIndentDecrease") {
                div { title: "MdFormatIndentDecrease",
                    Icon { icon: MdFormatIndentDecrease, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatIndentIncrease") {
                div { title: "MdFormatIndentIncrease",
                    Icon { icon: MdFormatIndentIncrease, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatItalic") {
                div { title: "MdFormatItalic",
                    Icon { icon: MdFormatItalic, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatLineSpacing") {
                div { title: "MdFormatLineSpacing",
                    Icon { icon: MdFormatLineSpacing, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatListBulleted") {
                div { title: "MdFormatListBulleted",
                    Icon { icon: MdFormatListBulleted, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatListNumbered") {
                div { title: "MdFormatListNumbered",
                    Icon { icon: MdFormatListNumbered, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatListNumberedRtl") {
                div { title: "MdFormatListNumberedRtl",
                    Icon {
                        icon: MdFormatListNumberedRtl,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdFormatPaint") {
                div { title: "MdFormatPaint",
                    Icon { icon: MdFormatPaint, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatQuote") {
                div { title: "MdFormatQuote",
                    Icon { icon: MdFormatQuote, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatShapes") {
                div { title: "MdFormatShapes",
                    Icon { icon: MdFormatShapes, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatSize") {
                div { title: "MdFormatSize",
                    Icon { icon: MdFormatSize, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatStrikethrough") {
                div { title: "MdFormatStrikethrough",
                    Icon { icon: MdFormatStrikethrough, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFormatTextdirectionLToR") {
                div { title: "MdFormatTextdirectionLToR",
                    Icon {
                        icon: MdFormatTextdirectionLToR,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdFormatTextdirectionRToL") {
                div { title: "MdFormatTextdirectionRToL",
                    Icon {
                        icon: MdFormatTextdirectionRToL,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdFormatUnderlined") {
                div { title: "MdFormatUnderlined",
                    Icon { icon: MdFormatUnderlined, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdForum") {
                div { title: "MdForum",
                    Icon { icon: MdForum, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdForward") {
                div { title: "MdForward",
                    Icon { icon: MdForward, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotG(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdForward5") {
                div { title: "MdForward5",
                    Icon { icon: MdForward5, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdForward10") {
                div { title: "MdForward10",
                    Icon { icon: MdForward10, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdForward30") {
                div { title: "MdForward30",
                    Icon { icon: MdForward30, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdForwardToInbox") {
                div { title: "MdForwardToInbox",
                    Icon { icon: MdForwardToInbox, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFoundation") {
                div { title: "MdFoundation",
                    Icon { icon: MdFoundation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFreeBreakfast") {
                div { title: "MdFreeBreakfast",
                    Icon { icon: MdFreeBreakfast, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFullscreen") {
                div { title: "MdFullscreen",
                    Icon { icon: MdFullscreen, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFullscreenExit") {
                div { title: "MdFullscreenExit",
                    Icon { icon: MdFullscreenExit, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdFunctions") {
                div { title: "MdFunctions",
                    Icon { icon: MdFunctions, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGamepad") {
                div { title: "MdGamepad",
                    Icon { icon: MdGamepad, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGames") {
                div { title: "MdGames",
                    Icon { icon: MdGames, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGavel") {
                div { title: "MdGavel",
                    Icon { icon: MdGavel, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGesture") {
                div { title: "MdGesture",
                    Icon { icon: MdGesture, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGetApp") {
                div { title: "MdGetApp",
                    Icon { icon: MdGetApp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGif") {
                div { title: "MdGif",
                    Icon { icon: MdGif, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGolfCourse") {
                div { title: "MdGolfCourse",
                    Icon { icon: MdGolfCourse, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGpsFixed") {
                div { title: "MdGpsFixed",
                    Icon { icon: MdGpsFixed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGpsNotFixed") {
                div { title: "MdGpsNotFixed",
                    Icon { icon: MdGpsNotFixed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGpsOff") {
                div { title: "MdGpsOff",
                    Icon { icon: MdGpsOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGrade") {
                div { title: "MdGrade",
                    Icon { icon: MdGrade, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGradient") {
                div { title: "MdGradient",
                    Icon { icon: MdGradient, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGrading") {
                div { title: "MdGrading",
                    Icon { icon: MdGrading, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGrain") {
                div { title: "MdGrain",
                    Icon { icon: MdGrain, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGraphicEq") {
                div { title: "MdGraphicEq",
                    Icon { icon: MdGraphicEq, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGrass") {
                div { title: "MdGrass",
                    Icon { icon: MdGrass, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGridOff") {
                div { title: "MdGridOff",
                    Icon { icon: MdGridOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGridOn") {
                div { title: "MdGridOn",
                    Icon { icon: MdGridOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGridView") {
                div { title: "MdGridView",
                    Icon { icon: MdGridView, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGroup") {
                div { title: "MdGroup",
                    Icon { icon: MdGroup, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGroupAdd") {
                div { title: "MdGroupAdd",
                    Icon { icon: MdGroupAdd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGroups") {
                div { title: "MdGroups",
                    Icon { icon: MdGroups, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGroupWork") {
                div { title: "MdGroupWork",
                    Icon { icon: MdGroupWork, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdGTranslate") {
                div { title: "MdGTranslate",
                    Icon { icon: MdGTranslate, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHail") {
                div { title: "MdHail",
                    Icon { icon: MdHail, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHandyman") {
                div { title: "MdHandyman",
                    Icon { icon: MdHandyman, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHardware") {
                div { title: "MdHardware",
                    Icon { icon: MdHardware, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHd") {
                div { title: "MdHd",
                    Icon { icon: MdHd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHdrEnhancedSelect") {
                div { title: "MdHdrEnhancedSelect",
                    Icon { icon: MdHdrEnhancedSelect, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHdrOff") {
                div { title: "MdHdrOff",
                    Icon { icon: MdHdrOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHdrOn") {
                div { title: "MdHdrOn",
                    Icon { icon: MdHdrOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHdrStrong") {
                div { title: "MdHdrStrong",
                    Icon { icon: MdHdrStrong, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHdrWeak") {
                div { title: "MdHdrWeak",
                    Icon { icon: MdHdrWeak, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHeadset") {
                div { title: "MdHeadset",
                    Icon { icon: MdHeadset, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHeadsetMic") {
                div { title: "MdHeadsetMic",
                    Icon { icon: MdHeadsetMic, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHeadsetOff") {
                div { title: "MdHeadsetOff",
                    Icon { icon: MdHeadsetOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHealing") {
                div { title: "MdHealing",
                    Icon { icon: MdHealing, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHearing") {
                div { title: "MdHearing",
                    Icon { icon: MdHearing, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHearingDisabled") {
                div { title: "MdHearingDisabled",
                    Icon { icon: MdHearingDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHeight") {
                div { title: "MdHeight",
                    Icon { icon: MdHeight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHelp") {
                div { title: "MdHelp",
                    Icon { icon: MdHelp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHelpCenter") {
                div { title: "MdHelpCenter",
                    Icon { icon: MdHelpCenter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHelpOutline") {
                div { title: "MdHelpOutline",
                    Icon { icon: MdHelpOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHighlight") {
                div { title: "MdHighlight",
                    Icon { icon: MdHighlight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHighlightAlt") {
                div { title: "MdHighlightAlt",
                    Icon { icon: MdHighlightAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHighlightOff") {
                div { title: "MdHighlightOff",
                    Icon { icon: MdHighlightOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHighQuality") {
                div { title: "MdHighQuality",
                    Icon { icon: MdHighQuality, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHistory") {
                div { title: "MdHistory",
                    Icon { icon: MdHistory, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHistoryEdu") {
                div { title: "MdHistoryEdu",
                    Icon { icon: MdHistoryEdu, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHistoryToggleOff") {
                div { title: "MdHistoryToggleOff",
                    Icon { icon: MdHistoryToggleOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHome") {
                div { title: "MdHome",
                    Icon { icon: MdHome, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHomeFilled") {
                div { title: "MdHomeFilled",
                    Icon { icon: MdHomeFilled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHomeRepairService") {
                div { title: "MdHomeRepairService",
                    Icon { icon: MdHomeRepairService, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHomeWork") {
                div { title: "MdHomeWork",
                    Icon { icon: MdHomeWork, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHorizontalRule") {
                div { title: "MdHorizontalRule",
                    Icon { icon: MdHorizontalRule, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHorizontalSplit") {
                div { title: "MdHorizontalSplit",
                    Icon { icon: MdHorizontalSplit, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHotel") {
                div { title: "MdHotel",
                    Icon { icon: MdHotel, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHotTub") {
                div { title: "MdHotTub",
                    Icon { icon: MdHotTub, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHourglassBottom") {
                div { title: "MdHourglassBottom",
                    Icon { icon: MdHourglassBottom, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHourglassDisabled") {
                div { title: "MdHourglassDisabled",
                    Icon { icon: MdHourglassDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHourglassEmpty") {
                div { title: "MdHourglassEmpty",
                    Icon { icon: MdHourglassEmpty, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHourglassFull") {
                div { title: "MdHourglassFull",
                    Icon { icon: MdHourglassFull, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHourglassTop") {
                div { title: "MdHourglassTop",
                    Icon { icon: MdHourglassTop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHouse") {
                div { title: "MdHouse",
                    Icon { icon: MdHouse, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHouseSiding") {
                div { title: "MdHouseSiding",
                    Icon { icon: MdHouseSiding, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHowToReg") {
                div { title: "MdHowToReg",
                    Icon { icon: MdHowToReg, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHowToVote") {
                div { title: "MdHowToVote",
                    Icon { icon: MdHowToVote, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHttp") {
                div { title: "MdHttp",
                    Icon { icon: MdHttp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHttps") {
                div { title: "MdHttps",
                    Icon { icon: MdHttps, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdHvac") {
                div { title: "MdHvac",
                    Icon { icon: MdHvac, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdIcecream") {
                div { title: "MdIcecream",
                    Icon { icon: MdIcecream, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdImage") {
                div { title: "MdImage",
                    Icon { icon: MdImage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdImageAspectRatio") {
                div { title: "MdImageAspectRatio",
                    Icon { icon: MdImageAspectRatio, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdImageNotSupported") {
                div { title: "MdImageNotSupported",
                    Icon { icon: MdImageNotSupported, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdImageSearch") {
                div { title: "MdImageSearch",
                    Icon { icon: MdImageSearch, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdImagesearchRoller") {
                div { title: "MdImagesearchRoller",
                    Icon { icon: MdImagesearchRoller, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdImportantDevices") {
                div { title: "MdImportantDevices",
                    Icon { icon: MdImportantDevices, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdImportContacts") {
                div { title: "MdImportContacts",
                    Icon { icon: MdImportContacts, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdImportExport") {
                div { title: "MdImportExport",
                    Icon { icon: MdImportExport, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInbox") {
                div { title: "MdInbox",
                    Icon { icon: MdInbox, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdIndeterminateCheckBox") {
                div { title: "MdIndeterminateCheckBox",
                    Icon {
                        icon: MdIndeterminateCheckBox,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdInfo") {
                div { title: "MdInfo",
                    Icon { icon: MdInfo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInfoOutline") {
                div { title: "MdInfoOutline",
                    Icon { icon: MdInfoOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInput") {
                div { title: "MdInput",
                    Icon { icon: MdInput, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInsertChart") {
                div { title: "MdInsertChart",
                    Icon { icon: MdInsertChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInsertChartOutlined") {
                div { title: "MdInsertChartOutlined",
                    Icon { icon: MdInsertChartOutlined, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInsertComment") {
                div { title: "MdInsertComment",
                    Icon { icon: MdInsertComment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInsertDriveFile") {
                div { title: "MdInsertDriveFile",
                    Icon { icon: MdInsertDriveFile, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInsertEmoticon") {
                div { title: "MdInsertEmoticon",
                    Icon { icon: MdInsertEmoticon, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInsertInvitation") {
                div { title: "MdInsertInvitation",
                    Icon { icon: MdInsertInvitation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInsertLink") {
                div { title: "MdInsertLink",
                    Icon { icon: MdInsertLink, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotH(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdInsertPhoto") {
                div { title: "MdInsertPhoto",
                    Icon { icon: MdInsertPhoto, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInsights") {
                div { title: "MdInsights",
                    Icon { icon: MdInsights, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdIntegrationInstructions") {
                div { title: "MdIntegrationInstructions",
                    Icon {
                        icon: MdIntegrationInstructions,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdInventory") {
                div { title: "MdInventory",
                    Icon { icon: MdInventory, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInvertColors") {
                div { title: "MdInvertColors",
                    Icon { icon: MdInvertColors, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdInvertColorsOff") {
                div { title: "MdInvertColorsOff",
                    Icon { icon: MdInvertColorsOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdIosShare") {
                div { title: "MdIosShare",
                    Icon { icon: MdIosShare, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdIso") {
                div { title: "MdIso",
                    Icon { icon: MdIso, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboard") {
                div { title: "MdKeyboard",
                    Icon { icon: MdKeyboard, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboardArrowDown") {
                div { title: "MdKeyboardArrowDown",
                    Icon { icon: MdKeyboardArrowDown, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboardArrowLeft") {
                div { title: "MdKeyboardArrowLeft",
                    Icon { icon: MdKeyboardArrowLeft, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboardArrowRight") {
                div { title: "MdKeyboardArrowRight",
                    Icon { icon: MdKeyboardArrowRight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboardArrowUp") {
                div { title: "MdKeyboardArrowUp",
                    Icon { icon: MdKeyboardArrowUp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboardBackspace") {
                div { title: "MdKeyboardBackspace",
                    Icon { icon: MdKeyboardBackspace, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboardCapslock") {
                div { title: "MdKeyboardCapslock",
                    Icon { icon: MdKeyboardCapslock, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboardHide") {
                div { title: "MdKeyboardHide",
                    Icon { icon: MdKeyboardHide, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboardReturn") {
                div { title: "MdKeyboardReturn",
                    Icon { icon: MdKeyboardReturn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboardTab") {
                div { title: "MdKeyboardTab",
                    Icon { icon: MdKeyboardTab, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKeyboardVoice") {
                div { title: "MdKeyboardVoice",
                    Icon { icon: MdKeyboardVoice, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKingBed") {
                div { title: "MdKingBed",
                    Icon { icon: MdKingBed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdKitchen") {
                div { title: "MdKitchen",
                    Icon { icon: MdKitchen, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLabel") {
                div { title: "MdLabel",
                    Icon { icon: MdLabel, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLabelImportant") {
                div { title: "MdLabelImportant",
                    Icon { icon: MdLabelImportant, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLabelImportantOutline") {
                div { title: "MdLabelImportantOutline",
                    Icon {
                        icon: MdLabelImportantOutline,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdLabelOff") {
                div { title: "MdLabelOff",
                    Icon { icon: MdLabelOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLabelOutline") {
                div { title: "MdLabelOutline",
                    Icon { icon: MdLabelOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLandscape") {
                div { title: "MdLandscape",
                    Icon { icon: MdLandscape, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLanguage") {
                div { title: "MdLanguage",
                    Icon { icon: MdLanguage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLaptop") {
                div { title: "MdLaptop",
                    Icon { icon: MdLaptop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLaptopChromebook") {
                div { title: "MdLaptopChromebook",
                    Icon { icon: MdLaptopChromebook, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLaptopMac") {
                div { title: "MdLaptopMac",
                    Icon { icon: MdLaptopMac, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLaptopWindows") {
                div { title: "MdLaptopWindows",
                    Icon { icon: MdLaptopWindows, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLastPage") {
                div { title: "MdLastPage",
                    Icon { icon: MdLastPage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLaunch") {
                div { title: "MdLaunch",
                    Icon { icon: MdLaunch, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLayers") {
                div { title: "MdLayers",
                    Icon { icon: MdLayers, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLayersClear") {
                div { title: "MdLayersClear",
                    Icon { icon: MdLayersClear, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLeaderboard") {
                div { title: "MdLeaderboard",
                    Icon { icon: MdLeaderboard, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLeakAdd") {
                div { title: "MdLeakAdd",
                    Icon { icon: MdLeakAdd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLeakRemove") {
                div { title: "MdLeakRemove",
                    Icon { icon: MdLeakRemove, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLegendToggle") {
                div { title: "MdLegendToggle",
                    Icon { icon: MdLegendToggle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLens") {
                div { title: "MdLens",
                    Icon { icon: MdLens, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLibraryAdd") {
                div { title: "MdLibraryAdd",
                    Icon { icon: MdLibraryAdd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLibraryAddCheck") {
                div { title: "MdLibraryAddCheck",
                    Icon { icon: MdLibraryAddCheck, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLibraryBooks") {
                div { title: "MdLibraryBooks",
                    Icon { icon: MdLibraryBooks, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLibraryMusic") {
                div { title: "MdLibraryMusic",
                    Icon { icon: MdLibraryMusic, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLightbulb") {
                div { title: "MdLightbulb",
                    Icon { icon: MdLightbulb, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLightbulbOutline") {
                div { title: "MdLightbulbOutline",
                    Icon { icon: MdLightbulbOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLinearScale") {
                div { title: "MdLinearScale",
                    Icon { icon: MdLinearScale, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLineStyle") {
                div { title: "MdLineStyle",
                    Icon { icon: MdLineStyle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLineWeight") {
                div { title: "MdLineWeight",
                    Icon { icon: MdLineWeight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLink") {
                div { title: "MdLink",
                    Icon { icon: MdLink, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLinkedCamera") {
                div { title: "MdLinkedCamera",
                    Icon { icon: MdLinkedCamera, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLinkOff") {
                div { title: "MdLinkOff",
                    Icon { icon: MdLinkOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLiquor") {
                div { title: "MdLiquor",
                    Icon { icon: MdLiquor, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdList") {
                div { title: "MdList",
                    Icon { icon: MdList, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdListAlt") {
                div { title: "MdListAlt",
                    Icon { icon: MdListAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLiveHelp") {
                div { title: "MdLiveHelp",
                    Icon { icon: MdLiveHelp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLiveTv") {
                div { title: "MdLiveTv",
                    Icon { icon: MdLiveTv, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalActivity") {
                div { title: "MdLocalActivity",
                    Icon { icon: MdLocalActivity, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalAirport") {
                div { title: "MdLocalAirport",
                    Icon { icon: MdLocalAirport, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalAtm") {
                div { title: "MdLocalAtm",
                    Icon { icon: MdLocalAtm, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalBar") {
                div { title: "MdLocalBar",
                    Icon { icon: MdLocalBar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalCafe") {
                div { title: "MdLocalCafe",
                    Icon { icon: MdLocalCafe, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalCarWash") {
                div { title: "MdLocalCarWash",
                    Icon { icon: MdLocalCarWash, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalConvenienceStore") {
                div { title: "MdLocalConvenienceStore",
                    Icon {
                        icon: MdLocalConvenienceStore,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdLocalDining") {
                div { title: "MdLocalDining",
                    Icon { icon: MdLocalDining, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalDrink") {
                div { title: "MdLocalDrink",
                    Icon { icon: MdLocalDrink, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalFireDepartment") {
                div { title: "MdLocalFireDepartment",
                    Icon { icon: MdLocalFireDepartment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalFlorist") {
                div { title: "MdLocalFlorist",
                    Icon { icon: MdLocalFlorist, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalGasStation") {
                div { title: "MdLocalGasStation",
                    Icon { icon: MdLocalGasStation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalGroceryStore") {
                div { title: "MdLocalGroceryStore",
                    Icon { icon: MdLocalGroceryStore, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalHospital") {
                div { title: "MdLocalHospital",
                    Icon { icon: MdLocalHospital, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalHotel") {
                div { title: "MdLocalHotel",
                    Icon { icon: MdLocalHotel, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalLaundryService") {
                div { title: "MdLocalLaundryService",
                    Icon { icon: MdLocalLaundryService, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalLibrary") {
                div { title: "MdLocalLibrary",
                    Icon { icon: MdLocalLibrary, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalMall") {
                div { title: "MdLocalMall",
                    Icon { icon: MdLocalMall, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalMovies") {
                div { title: "MdLocalMovies",
                    Icon { icon: MdLocalMovies, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalOffer") {
                div { title: "MdLocalOffer",
                    Icon { icon: MdLocalOffer, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalParking") {
                div { title: "MdLocalParking",
                    Icon { icon: MdLocalParking, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalPharmacy") {
                div { title: "MdLocalPharmacy",
                    Icon { icon: MdLocalPharmacy, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalPhone") {
                div { title: "MdLocalPhone",
                    Icon { icon: MdLocalPhone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalPizza") {
                div { title: "MdLocalPizza",
                    Icon { icon: MdLocalPizza, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalPlay") {
                div { title: "MdLocalPlay",
                    Icon { icon: MdLocalPlay, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalPolice") {
                div { title: "MdLocalPolice",
                    Icon { icon: MdLocalPolice, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalPostOffice") {
                div { title: "MdLocalPostOffice",
                    Icon { icon: MdLocalPostOffice, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalPrintshop") {
                div { title: "MdLocalPrintshop",
                    Icon { icon: MdLocalPrintshop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalSee") {
                div { title: "MdLocalSee",
                    Icon { icon: MdLocalSee, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalShipping") {
                div { title: "MdLocalShipping",
                    Icon { icon: MdLocalShipping, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocalTaxi") {
                div { title: "MdLocalTaxi",
                    Icon { icon: MdLocalTaxi, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocationCity") {
                div { title: "MdLocationCity",
                    Icon { icon: MdLocationCity, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocationDisabled") {
                div { title: "MdLocationDisabled",
                    Icon { icon: MdLocationDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocationOff") {
                div { title: "MdLocationOff",
                    Icon { icon: MdLocationOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocationOn") {
                div { title: "MdLocationOn",
                    Icon { icon: MdLocationOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocationPin") {
                div { title: "MdLocationPin",
                    Icon { icon: MdLocationPin, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLocationSearching") {
                div { title: "MdLocationSearching",
                    Icon { icon: MdLocationSearching, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLock") {
                div { title: "MdLock",
                    Icon { icon: MdLock, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLockClock") {
                div { title: "MdLockClock",
                    Icon { icon: MdLockClock, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLockOpen") {
                div { title: "MdLockOpen",
                    Icon { icon: MdLockOpen, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLockOutline") {
                div { title: "MdLockOutline",
                    Icon { icon: MdLockOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLogin") {
                div { title: "MdLogin",
                    Icon { icon: MdLogin, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotI(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdLogout") {
                div { title: "MdLogout",
                    Icon { icon: MdLogout, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLooks") {
                div { title: "MdLooks",
                    Icon { icon: MdLooks, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLooks3") {
                div { title: "MdLooks3",
                    Icon { icon: MdLooks3, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLooks4") {
                div { title: "MdLooks4",
                    Icon { icon: MdLooks4, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLooks5") {
                div { title: "MdLooks5",
                    Icon { icon: MdLooks5, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLooks6") {
                div { title: "MdLooks6",
                    Icon { icon: MdLooks6, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLooksOne") {
                div { title: "MdLooksOne",
                    Icon { icon: MdLooksOne, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLooksTwo") {
                div { title: "MdLooksTwo",
                    Icon { icon: MdLooksTwo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLoop") {
                div { title: "MdLoop",
                    Icon { icon: MdLoop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLoupe") {
                div { title: "MdLoupe",
                    Icon { icon: MdLoupe, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLowPriority") {
                div { title: "MdLowPriority",
                    Icon { icon: MdLowPriority, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLoyalty") {
                div { title: "MdLoyalty",
                    Icon { icon: MdLoyalty, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLuggage") {
                div { title: "MdLuggage",
                    Icon { icon: MdLuggage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdLunchDining") {
                div { title: "MdLunchDining",
                    Icon { icon: MdLunchDining, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMail") {
                div { title: "MdMail",
                    Icon { icon: MdMail, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMailOutline") {
                div { title: "MdMailOutline",
                    Icon { icon: MdMailOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMap") {
                div { title: "MdMap",
                    Icon { icon: MdMap, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMapsUgc") {
                div { title: "MdMapsUgc",
                    Icon { icon: MdMapsUgc, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMargin") {
                div { title: "MdMargin",
                    Icon { icon: MdMargin, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMarkAsUnread") {
                div { title: "MdMarkAsUnread",
                    Icon { icon: MdMarkAsUnread, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMarkChatRead") {
                div { title: "MdMarkChatRead",
                    Icon { icon: MdMarkChatRead, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMarkChatUnread") {
                div { title: "MdMarkChatUnread",
                    Icon { icon: MdMarkChatUnread, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMarkEmailRead") {
                div { title: "MdMarkEmailRead",
                    Icon { icon: MdMarkEmailRead, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMarkEmailUnread") {
                div { title: "MdMarkEmailUnread",
                    Icon { icon: MdMarkEmailUnread, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMarkunread") {
                div { title: "MdMarkunread",
                    Icon { icon: MdMarkunread, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMarkunreadMailbox") {
                div { title: "MdMarkunreadMailbox",
                    Icon { icon: MdMarkunreadMailbox, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMasks") {
                div { title: "MdMasks",
                    Icon { icon: MdMasks, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMaximize") {
                div { title: "MdMaximize",
                    Icon { icon: MdMaximize, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMediation") {
                div { title: "MdMediation",
                    Icon { icon: MdMediation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMedicalServices") {
                div { title: "MdMedicalServices",
                    Icon { icon: MdMedicalServices, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMeetingRoom") {
                div { title: "MdMeetingRoom",
                    Icon { icon: MdMeetingRoom, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMemory") {
                div { title: "MdMemory",
                    Icon { icon: MdMemory, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMenu") {
                div { title: "MdMenu",
                    Icon { icon: MdMenu, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMenuBook") {
                div { title: "MdMenuBook",
                    Icon { icon: MdMenuBook, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMenuOpen") {
                div { title: "MdMenuOpen",
                    Icon { icon: MdMenuOpen, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMergeType") {
                div { title: "MdMergeType",
                    Icon { icon: MdMergeType, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMessage") {
                div { title: "MdMessage",
                    Icon { icon: MdMessage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMic") {
                div { title: "MdMic",
                    Icon { icon: MdMic, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMicExternalOff") {
                div { title: "MdMicExternalOff",
                    Icon { icon: MdMicExternalOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMicExternalOn") {
                div { title: "MdMicExternalOn",
                    Icon { icon: MdMicExternalOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMicNone") {
                div { title: "MdMicNone",
                    Icon { icon: MdMicNone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMicOff") {
                div { title: "MdMicOff",
                    Icon { icon: MdMicOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMicrowave") {
                div { title: "MdMicrowave",
                    Icon { icon: MdMicrowave, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMilitaryTech") {
                div { title: "MdMilitaryTech",
                    Icon { icon: MdMilitaryTech, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMinimize") {
                div { title: "MdMinimize",
                    Icon { icon: MdMinimize, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMiscellaneousServices") {
                div { title: "MdMiscellaneousServices",
                    Icon {
                        icon: MdMiscellaneousServices,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdMissedVideoCall") {
                div { title: "MdMissedVideoCall",
                    Icon { icon: MdMissedVideoCall, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMms") {
                div { title: "MdMms",
                    Icon { icon: MdMms, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMobileFriendly") {
                div { title: "MdMobileFriendly",
                    Icon { icon: MdMobileFriendly, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMobileOff") {
                div { title: "MdMobileOff",
                    Icon { icon: MdMobileOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMobileScreenShare") {
                div { title: "MdMobileScreenShare",
                    Icon { icon: MdMobileScreenShare, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdModeComment") {
                div { title: "MdModeComment",
                    Icon { icon: MdModeComment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdModeEdit") {
                div { title: "MdModeEdit",
                    Icon { icon: MdModeEdit, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdModelTraining") {
                div { title: "MdModelTraining",
                    Icon { icon: MdModelTraining, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMonetizationOn") {
                div { title: "MdMonetizationOn",
                    Icon { icon: MdMonetizationOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMoney") {
                div { title: "MdMoney",
                    Icon { icon: MdMoney, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMoneyOff") {
                div { title: "MdMoneyOff",
                    Icon { icon: MdMoneyOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMonitor") {
                div { title: "MdMonitor",
                    Icon { icon: MdMonitor, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMonochromePhotos") {
                div { title: "MdMonochromePhotos",
                    Icon { icon: MdMonochromePhotos, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMood") {
                div { title: "MdMood",
                    Icon { icon: MdMood, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMoodBad") {
                div { title: "MdMoodBad",
                    Icon { icon: MdMoodBad, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMoped") {
                div { title: "MdMoped",
                    Icon { icon: MdMoped, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMore") {
                div { title: "MdMore",
                    Icon { icon: MdMore, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMoreHoriz") {
                div { title: "MdMoreHoriz",
                    Icon { icon: MdMoreHoriz, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMoreTime") {
                div { title: "MdMoreTime",
                    Icon { icon: MdMoreTime, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMoreVert") {
                div { title: "MdMoreVert",
                    Icon { icon: MdMoreVert, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMotionPhotosOff") {
                div { title: "MdMotionPhotosOff",
                    Icon { icon: MdMotionPhotosOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMotionPhotosOn") {
                div { title: "MdMotionPhotosOn",
                    Icon { icon: MdMotionPhotosOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMotionPhotosPause") {
                div { title: "MdMotionPhotosPause",
                    Icon { icon: MdMotionPhotosPause, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMotionPhotosPaused") {
                div { title: "MdMotionPhotosPaused",
                    Icon { icon: MdMotionPhotosPaused, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMouse") {
                div { title: "MdMouse",
                    Icon { icon: MdMouse, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMoveToInbox") {
                div { title: "MdMoveToInbox",
                    Icon { icon: MdMoveToInbox, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMovie") {
                div { title: "MdMovie",
                    Icon { icon: MdMovie, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMovieCreation") {
                div { title: "MdMovieCreation",
                    Icon { icon: MdMovieCreation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMovieFilter") {
                div { title: "MdMovieFilter",
                    Icon { icon: MdMovieFilter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMp") {
                div { title: "MdMp",
                    Icon { icon: MdMp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMultilineChart") {
                div { title: "MdMultilineChart",
                    Icon { icon: MdMultilineChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMultipleStop") {
                div { title: "MdMultipleStop",
                    Icon { icon: MdMultipleStop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMuseum") {
                div { title: "MdMuseum",
                    Icon { icon: MdMuseum, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMusicNote") {
                div { title: "MdMusicNote",
                    Icon { icon: MdMusicNote, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMusicOff") {
                div { title: "MdMusicOff",
                    Icon { icon: MdMusicOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMusicVideo") {
                div { title: "MdMusicVideo",
                    Icon { icon: MdMusicVideo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdMyLocation") {
                div { title: "MdMyLocation",
                    Icon { icon: MdMyLocation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNat") {
                div { title: "MdNat",
                    Icon { icon: MdNat, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNature") {
                div { title: "MdNature",
                    Icon { icon: MdNature, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNaturePeople") {
                div { title: "MdNaturePeople",
                    Icon { icon: MdNaturePeople, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNavigateBefore") {
                div { title: "MdNavigateBefore",
                    Icon { icon: MdNavigateBefore, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNavigateNext") {
                div { title: "MdNavigateNext",
                    Icon { icon: MdNavigateNext, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNavigation") {
                div { title: "MdNavigation",
                    Icon { icon: MdNavigation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNearMe") {
                div { title: "MdNearMe",
                    Icon { icon: MdNearMe, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNearMeDisabled") {
                div { title: "MdNearMeDisabled",
                    Icon { icon: MdNearMeDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNetworkCell") {
                div { title: "MdNetworkCell",
                    Icon { icon: MdNetworkCell, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNetworkCheck") {
                div { title: "MdNetworkCheck",
                    Icon { icon: MdNetworkCheck, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNetworkLocked") {
                div { title: "MdNetworkLocked",
                    Icon { icon: MdNetworkLocked, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNetworkWifi") {
                div { title: "MdNetworkWifi",
                    Icon { icon: MdNetworkWifi, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNewReleases") {
                div { title: "MdNewReleases",
                    Icon { icon: MdNewReleases, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNextPlan") {
                div { title: "MdNextPlan",
                    Icon { icon: MdNextPlan, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNextWeek") {
                div { title: "MdNextWeek",
                    Icon { icon: MdNextWeek, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNfc") {
                div { title: "MdNfc",
                    Icon { icon: MdNfc, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNightlife") {
                div { title: "MdNightlife",
                    Icon { icon: MdNightlife, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotJ(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdNightlightRound") {
                div { title: "MdNightlightRound",
                    Icon { icon: MdNightlightRound, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNightShelter") {
                div { title: "MdNightShelter",
                    Icon { icon: MdNightShelter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNightsStay") {
                div { title: "MdNightsStay",
                    Icon { icon: MdNightsStay, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoBackpack") {
                div { title: "MdNoBackpack",
                    Icon { icon: MdNoBackpack, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoCell") {
                div { title: "MdNoCell",
                    Icon { icon: MdNoCell, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoDrinks") {
                div { title: "MdNoDrinks",
                    Icon { icon: MdNoDrinks, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoEncryption") {
                div { title: "MdNoEncryption",
                    Icon { icon: MdNoEncryption, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoFlash") {
                div { title: "MdNoFlash",
                    Icon { icon: MdNoFlash, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoFood") {
                div { title: "MdNoFood",
                    Icon { icon: MdNoFood, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoLuggage") {
                div { title: "MdNoLuggage",
                    Icon { icon: MdNoLuggage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoMeals") {
                div { title: "MdNoMeals",
                    Icon { icon: MdNoMeals, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoMealsOuline") {
                div { title: "MdNoMealsOuline",
                    Icon { icon: MdNoMealsOuline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoMeetingRoom") {
                div { title: "MdNoMeetingRoom",
                    Icon { icon: MdNoMeetingRoom, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoPhotography") {
                div { title: "MdNoPhotography",
                    Icon { icon: MdNoPhotography, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNorth") {
                div { title: "MdNorth",
                    Icon { icon: MdNorth, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNorthEast") {
                div { title: "MdNorthEast",
                    Icon { icon: MdNorthEast, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNorthWest") {
                div { title: "MdNorthWest",
                    Icon { icon: MdNorthWest, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoSim") {
                div { title: "MdNoSim",
                    Icon { icon: MdNoSim, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoStroller") {
                div { title: "MdNoStroller",
                    Icon { icon: MdNoStroller, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNotAccessible") {
                div { title: "MdNotAccessible",
                    Icon { icon: MdNotAccessible, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNote") {
                div { title: "MdNote",
                    Icon { icon: MdNote, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoteAdd") {
                div { title: "MdNoteAdd",
                    Icon { icon: MdNoteAdd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNotes") {
                div { title: "MdNotes",
                    Icon { icon: MdNotes, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNotificationImportant") {
                div { title: "MdNotificationImportant",
                    Icon {
                        icon: MdNotificationImportant,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdNotifications") {
                div { title: "MdNotifications",
                    Icon { icon: MdNotifications, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNotificationsActive") {
                div { title: "MdNotificationsActive",
                    Icon { icon: MdNotificationsActive, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNotificationsNone") {
                div { title: "MdNotificationsNone",
                    Icon { icon: MdNotificationsNone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNotificationsOff") {
                div { title: "MdNotificationsOff",
                    Icon { icon: MdNotificationsOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNotificationsPaused") {
                div { title: "MdNotificationsPaused",
                    Icon { icon: MdNotificationsPaused, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNotInterested") {
                div { title: "MdNotInterested",
                    Icon { icon: MdNotInterested, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNotListedLocation") {
                div { title: "MdNotListedLocation",
                    Icon { icon: MdNotListedLocation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNoTransfer") {
                div { title: "MdNoTransfer",
                    Icon { icon: MdNoTransfer, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdNotStarted") {
                div { title: "MdNotStarted",
                    Icon { icon: MdNotStarted, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOfflineBolt") {
                div { title: "MdOfflineBolt",
                    Icon { icon: MdOfflineBolt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOfflinePin") {
                div { title: "MdOfflinePin",
                    Icon { icon: MdOfflinePin, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOfflineShare") {
                div { title: "MdOfflineShare",
                    Icon { icon: MdOfflineShare, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOndemandVideo") {
                div { title: "MdOndemandVideo",
                    Icon { icon: MdOndemandVideo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOnlinePrediction") {
                div { title: "MdOnlinePrediction",
                    Icon { icon: MdOnlinePrediction, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOpacity") {
                div { title: "MdOpacity",
                    Icon { icon: MdOpacity, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOpenInBrowser") {
                div { title: "MdOpenInBrowser",
                    Icon { icon: MdOpenInBrowser, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOpenInFull") {
                div { title: "MdOpenInFull",
                    Icon { icon: MdOpenInFull, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOpenInNew") {
                div { title: "MdOpenInNew",
                    Icon { icon: MdOpenInNew, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOpenWith") {
                div { title: "MdOpenWith",
                    Icon { icon: MdOpenWith, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOutbond") {
                div { title: "MdOutbond",
                    Icon { icon: MdOutbond, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOutbox") {
                div { title: "MdOutbox",
                    Icon { icon: MdOutbox, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOutdoorGrill") {
                div { title: "MdOutdoorGrill",
                    Icon { icon: MdOutdoorGrill, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOutgoingMail") {
                div { title: "MdOutgoingMail",
                    Icon { icon: MdOutgoingMail, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOutlet") {
                div { title: "MdOutlet",
                    Icon { icon: MdOutlet, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdOutlinedFlag") {
                div { title: "MdOutlinedFlag",
                    Icon { icon: MdOutlinedFlag, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPadding") {
                div { title: "MdPadding",
                    Icon { icon: MdPadding, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPages") {
                div { title: "MdPages",
                    Icon { icon: MdPages, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPageview") {
                div { title: "MdPageview",
                    Icon { icon: MdPageview, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPalette") {
                div { title: "MdPalette",
                    Icon { icon: MdPalette, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPanorama") {
                div { title: "MdPanorama",
                    Icon { icon: MdPanorama, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPanoramaFishEye") {
                div { title: "MdPanoramaFishEye",
                    Icon { icon: MdPanoramaFishEye, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPanoramaHorizontal") {
                div { title: "MdPanoramaHorizontal",
                    Icon { icon: MdPanoramaHorizontal, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPanoramaHorizontalSelect") {
                div { title: "MdPanoramaHorizontalSelect",
                    Icon {
                        icon: MdPanoramaHorizontalSelect,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdPanoramaPhotosphere") {
                div { title: "MdPanoramaPhotosphere",
                    Icon { icon: MdPanoramaPhotosphere, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPanoramaPhotosphereSelect") {
                div { title: "MdPanoramaPhotosphereSelect",
                    Icon {
                        icon: MdPanoramaPhotosphereSelect,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdPanoramaVertical") {
                div { title: "MdPanoramaVertical",
                    Icon { icon: MdPanoramaVertical, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPanoramaVerticalSelect") {
                div { title: "MdPanoramaVerticalSelect",
                    Icon {
                        icon: MdPanoramaVerticalSelect,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdPanoramaWideAngle") {
                div { title: "MdPanoramaWideAngle",
                    Icon { icon: MdPanoramaWideAngle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPanoramaWideAngleSelect") {
                div { title: "MdPanoramaWideAngleSelect",
                    Icon {
                        icon: MdPanoramaWideAngleSelect,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdPanTool") {
                div { title: "MdPanTool",
                    Icon { icon: MdPanTool, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPark") {
                div { title: "MdPark",
                    Icon { icon: MdPark, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPartyMode") {
                div { title: "MdPartyMode",
                    Icon { icon: MdPartyMode, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPause") {
                div { title: "MdPause",
                    Icon { icon: MdPause, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPauseCircleFilled") {
                div { title: "MdPauseCircleFilled",
                    Icon { icon: MdPauseCircleFilled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPauseCircleOutline") {
                div { title: "MdPauseCircleOutline",
                    Icon { icon: MdPauseCircleOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPausePresentation") {
                div { title: "MdPausePresentation",
                    Icon { icon: MdPausePresentation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPayment") {
                div { title: "MdPayment",
                    Icon { icon: MdPayment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPayments") {
                div { title: "MdPayments",
                    Icon { icon: MdPayments, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPedalBike") {
                div { title: "MdPedalBike",
                    Icon { icon: MdPedalBike, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPending") {
                div { title: "MdPending",
                    Icon { icon: MdPending, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPendingActions") {
                div { title: "MdPendingActions",
                    Icon { icon: MdPendingActions, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPeople") {
                div { title: "MdPeople",
                    Icon { icon: MdPeople, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPeopleAlt") {
                div { title: "MdPeopleAlt",
                    Icon { icon: MdPeopleAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPeopleOutline") {
                div { title: "MdPeopleOutline",
                    Icon { icon: MdPeopleOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPermCameraMic") {
                div { title: "MdPermCameraMic",
                    Icon { icon: MdPermCameraMic, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPermContactCalendar") {
                div { title: "MdPermContactCalendar",
                    Icon { icon: MdPermContactCalendar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPermDataSetting") {
                div { title: "MdPermDataSetting",
                    Icon { icon: MdPermDataSetting, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPermDeviceInformation") {
                div { title: "MdPermDeviceInformation",
                    Icon {
                        icon: MdPermDeviceInformation,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdPermIdentity") {
                div { title: "MdPermIdentity",
                    Icon { icon: MdPermIdentity, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPermMedia") {
                div { title: "MdPermMedia",
                    Icon { icon: MdPermMedia, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPermPhoneMsg") {
                div { title: "MdPermPhoneMsg",
                    Icon { icon: MdPermPhoneMsg, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPermScanWifi") {
                div { title: "MdPermScanWifi",
                    Icon { icon: MdPermScanWifi, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPerson") {
                div { title: "MdPerson",
                    Icon { icon: MdPerson, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonAdd") {
                div { title: "MdPersonAdd",
                    Icon { icon: MdPersonAdd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonAddAlt") {
                div { title: "MdPersonAddAlt",
                    Icon { icon: MdPersonAddAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonAddAlt1") {
                div { title: "MdPersonAddAlt1",
                    Icon { icon: MdPersonAddAlt1, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonAddDisabled") {
                div { title: "MdPersonAddDisabled",
                    Icon { icon: MdPersonAddDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonalVideo") {
                div { title: "MdPersonalVideo",
                    Icon { icon: MdPersonalVideo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonOutline") {
                div { title: "MdPersonOutline",
                    Icon { icon: MdPersonOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonPin") {
                div { title: "MdPersonPin",
                    Icon { icon: MdPersonPin, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonPinCircle") {
                div { title: "MdPersonPinCircle",
                    Icon { icon: MdPersonPinCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonRemove") {
                div { title: "MdPersonRemove",
                    Icon { icon: MdPersonRemove, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonRemoveAlt1") {
                div { title: "MdPersonRemoveAlt1",
                    Icon { icon: MdPersonRemoveAlt1, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPersonSearch") {
                div { title: "MdPersonSearch",
                    Icon { icon: MdPersonSearch, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPestControl") {
                div { title: "MdPestControl",
                    Icon { icon: MdPestControl, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPestControlRodent") {
                div { title: "MdPestControlRodent",
                    Icon { icon: MdPestControlRodent, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotK(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdPets") {
                div { title: "MdPets",
                    Icon { icon: MdPets, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhone") {
                div { title: "MdPhone",
                    Icon { icon: MdPhone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhoneAndroid") {
                div { title: "MdPhoneAndroid",
                    Icon { icon: MdPhoneAndroid, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhoneBluetoothSpeaker") {
                div { title: "MdPhoneBluetoothSpeaker",
                    Icon {
                        icon: MdPhoneBluetoothSpeaker,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdPhoneCallback") {
                div { title: "MdPhoneCallback",
                    Icon { icon: MdPhoneCallback, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhoneDisabled") {
                div { title: "MdPhoneDisabled",
                    Icon { icon: MdPhoneDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhoneEnabled") {
                div { title: "MdPhoneEnabled",
                    Icon { icon: MdPhoneEnabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhoneForwarded") {
                div { title: "MdPhoneForwarded",
                    Icon { icon: MdPhoneForwarded, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhoneInTalk") {
                div { title: "MdPhoneInTalk",
                    Icon { icon: MdPhoneInTalk, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhoneIphone") {
                div { title: "MdPhoneIphone",
                    Icon { icon: MdPhoneIphone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhonelink") {
                div { title: "MdPhonelink",
                    Icon { icon: MdPhonelink, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhonelinkErase") {
                div { title: "MdPhonelinkErase",
                    Icon { icon: MdPhonelinkErase, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhonelinkLock") {
                div { title: "MdPhonelinkLock",
                    Icon { icon: MdPhonelinkLock, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhonelinkOff") {
                div { title: "MdPhonelinkOff",
                    Icon { icon: MdPhonelinkOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhonelinkRing") {
                div { title: "MdPhonelinkRing",
                    Icon { icon: MdPhonelinkRing, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhonelinkSetup") {
                div { title: "MdPhonelinkSetup",
                    Icon { icon: MdPhonelinkSetup, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhoneLocked") {
                div { title: "MdPhoneLocked",
                    Icon { icon: MdPhoneLocked, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhoneMissed") {
                div { title: "MdPhoneMissed",
                    Icon { icon: MdPhoneMissed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhonePaused") {
                div { title: "MdPhonePaused",
                    Icon { icon: MdPhonePaused, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhoto") {
                div { title: "MdPhoto",
                    Icon { icon: MdPhoto, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhotoAlbum") {
                div { title: "MdPhotoAlbum",
                    Icon { icon: MdPhotoAlbum, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhotoCamera") {
                div { title: "MdPhotoCamera",
                    Icon { icon: MdPhotoCamera, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhotoCameraBack") {
                div { title: "MdPhotoCameraBack",
                    Icon { icon: MdPhotoCameraBack, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhotoCameraFront") {
                div { title: "MdPhotoCameraFront",
                    Icon { icon: MdPhotoCameraFront, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhotoFilter") {
                div { title: "MdPhotoFilter",
                    Icon { icon: MdPhotoFilter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhotoLibrary") {
                div { title: "MdPhotoLibrary",
                    Icon { icon: MdPhotoLibrary, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhotoSizeSelectActual") {
                div { title: "MdPhotoSizeSelectActual",
                    Icon {
                        icon: MdPhotoSizeSelectActual,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdPhotoSizeSelectLarge") {
                div { title: "MdPhotoSizeSelectLarge",
                    Icon { icon: MdPhotoSizeSelectLarge, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPhotoSizeSelectSmall") {
                div { title: "MdPhotoSizeSelectSmall",
                    Icon { icon: MdPhotoSizeSelectSmall, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPictureAsPdf") {
                div { title: "MdPictureAsPdf",
                    Icon { icon: MdPictureAsPdf, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPictureInPicture") {
                div { title: "MdPictureInPicture",
                    Icon { icon: MdPictureInPicture, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPictureInPictureAlt") {
                div { title: "MdPictureInPictureAlt",
                    Icon { icon: MdPictureInPictureAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPieChart") {
                div { title: "MdPieChart",
                    Icon { icon: MdPieChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPieChartOutlined") {
                div { title: "MdPieChartOutlined",
                    Icon { icon: MdPieChartOutlined, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPinDrop") {
                div { title: "MdPinDrop",
                    Icon { icon: MdPinDrop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPivotTableChart") {
                div { title: "MdPivotTableChart",
                    Icon { icon: MdPivotTableChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlace") {
                div { title: "MdPlace",
                    Icon { icon: MdPlace, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlagiarism") {
                div { title: "MdPlagiarism",
                    Icon { icon: MdPlagiarism, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlayArrow") {
                div { title: "MdPlayArrow",
                    Icon { icon: MdPlayArrow, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlayCircleFilled") {
                div { title: "MdPlayCircleFilled",
                    Icon { icon: MdPlayCircleFilled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlayCircleOutline") {
                div { title: "MdPlayCircleOutline",
                    Icon { icon: MdPlayCircleOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlayDisabled") {
                div { title: "MdPlayDisabled",
                    Icon { icon: MdPlayDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlayForWork") {
                div { title: "MdPlayForWork",
                    Icon { icon: MdPlayForWork, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlaylistAdd") {
                div { title: "MdPlaylistAdd",
                    Icon { icon: MdPlaylistAdd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlaylistAddCheck") {
                div { title: "MdPlaylistAddCheck",
                    Icon { icon: MdPlaylistAddCheck, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlaylistPlay") {
                div { title: "MdPlaylistPlay",
                    Icon { icon: MdPlaylistPlay, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlumbing") {
                div { title: "MdPlumbing",
                    Icon { icon: MdPlumbing, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPlusOne") {
                div { title: "MdPlusOne",
                    Icon { icon: MdPlusOne, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPointOfSale") {
                div { title: "MdPointOfSale",
                    Icon { icon: MdPointOfSale, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPolicy") {
                div { title: "MdPolicy",
                    Icon { icon: MdPolicy, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPoll") {
                div { title: "MdPoll",
                    Icon { icon: MdPoll, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPolymer") {
                div { title: "MdPolymer",
                    Icon { icon: MdPolymer, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPool") {
                div { title: "MdPool",
                    Icon { icon: MdPool, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPortableWifiOff") {
                div { title: "MdPortableWifiOff",
                    Icon { icon: MdPortableWifiOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPortrait") {
                div { title: "MdPortrait",
                    Icon { icon: MdPortrait, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPostAdd") {
                div { title: "MdPostAdd",
                    Icon { icon: MdPostAdd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPower") {
                div { title: "MdPower",
                    Icon { icon: MdPower, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPowerInput") {
                div { title: "MdPowerInput",
                    Icon { icon: MdPowerInput, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPowerOff") {
                div { title: "MdPowerOff",
                    Icon { icon: MdPowerOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPowerSettingsNew") {
                div { title: "MdPowerSettingsNew",
                    Icon { icon: MdPowerSettingsNew, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPregnantWoman") {
                div { title: "MdPregnantWoman",
                    Icon { icon: MdPregnantWoman, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPresentToAll") {
                div { title: "MdPresentToAll",
                    Icon { icon: MdPresentToAll, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPreview") {
                div { title: "MdPreview",
                    Icon { icon: MdPreview, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPrint") {
                div { title: "MdPrint",
                    Icon { icon: MdPrint, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPrintDisabled") {
                div { title: "MdPrintDisabled",
                    Icon { icon: MdPrintDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPriorityHigh") {
                div { title: "MdPriorityHigh",
                    Icon { icon: MdPriorityHigh, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPrivacyTip") {
                div { title: "MdPrivacyTip",
                    Icon { icon: MdPrivacyTip, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPsychology") {
                div { title: "MdPsychology",
                    Icon { icon: MdPsychology, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPublic") {
                div { title: "MdPublic",
                    Icon { icon: MdPublic, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPublicOff") {
                div { title: "MdPublicOff",
                    Icon { icon: MdPublicOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPublish") {
                div { title: "MdPublish",
                    Icon { icon: MdPublish, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPublishedWithChanges") {
                div { title: "MdPublishedWithChanges",
                    Icon { icon: MdPublishedWithChanges, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdPushPin") {
                div { title: "MdPushPin",
                    Icon { icon: MdPushPin, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdQrCode") {
                div { title: "MdQrCode",
                    Icon { icon: MdQrCode, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdQrCodeScanner") {
                div { title: "MdQrCodeScanner",
                    Icon { icon: MdQrCodeScanner, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdQueryBuilder") {
                div { title: "MdQueryBuilder",
                    Icon { icon: MdQueryBuilder, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdQuestionAnswer") {
                div { title: "MdQuestionAnswer",
                    Icon { icon: MdQuestionAnswer, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdQueue") {
                div { title: "MdQueue",
                    Icon { icon: MdQueue, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdQueueMusic") {
                div { title: "MdQueueMusic",
                    Icon { icon: MdQueueMusic, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdQueuePlayNext") {
                div { title: "MdQueuePlayNext",
                    Icon { icon: MdQueuePlayNext, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdQuickreply") {
                div { title: "MdQuickreply",
                    Icon { icon: MdQuickreply, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRadio") {
                div { title: "MdRadio",
                    Icon { icon: MdRadio, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRadioButtonChecked") {
                div { title: "MdRadioButtonChecked",
                    Icon { icon: MdRadioButtonChecked, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRadioButtonUnchecked") {
                div { title: "MdRadioButtonUnchecked",
                    Icon { icon: MdRadioButtonUnchecked, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRailwayAlert") {
                div { title: "MdRailwayAlert",
                    Icon { icon: MdRailwayAlert, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRamenDining") {
                div { title: "MdRamenDining",
                    Icon { icon: MdRamenDining, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRateReview") {
                div { title: "MdRateReview",
                    Icon { icon: MdRateReview, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReadMore") {
                div { title: "MdReadMore",
                    Icon { icon: MdReadMore, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReceipt") {
                div { title: "MdReceipt",
                    Icon { icon: MdReceipt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReceiptLong") {
                div { title: "MdReceiptLong",
                    Icon { icon: MdReceiptLong, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRecentActors") {
                div { title: "MdRecentActors",
                    Icon { icon: MdRecentActors, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRecommend") {
                div { title: "MdRecommend",
                    Icon { icon: MdRecommend, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRecordVoiceOver") {
                div { title: "MdRecordVoiceOver",
                    Icon { icon: MdRecordVoiceOver, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRedeem") {
                div { title: "MdRedeem",
                    Icon { icon: MdRedeem, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRedo") {
                div { title: "MdRedo",
                    Icon { icon: MdRedo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReduceCapacity") {
                div { title: "MdReduceCapacity",
                    Icon { icon: MdReduceCapacity, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRefresh") {
                div { title: "MdRefresh",
                    Icon { icon: MdRefresh, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRemove") {
                div { title: "MdRemove",
                    Icon { icon: MdRemove, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRemoveCircle") {
                div { title: "MdRemoveCircle",
                    Icon { icon: MdRemoveCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRemoveCircleOutline") {
                div { title: "MdRemoveCircleOutline",
                    Icon { icon: MdRemoveCircleOutline, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotL(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdRemoveDone") {
                div { title: "MdRemoveDone",
                    Icon { icon: MdRemoveDone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRemoveFromQueue") {
                div { title: "MdRemoveFromQueue",
                    Icon { icon: MdRemoveFromQueue, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRemoveModerator") {
                div { title: "MdRemoveModerator",
                    Icon { icon: MdRemoveModerator, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRemoveRedEye") {
                div { title: "MdRemoveRedEye",
                    Icon { icon: MdRemoveRedEye, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRemoveShoppingCart") {
                div { title: "MdRemoveShoppingCart",
                    Icon { icon: MdRemoveShoppingCart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReorder") {
                div { title: "MdReorder",
                    Icon { icon: MdReorder, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRepeat") {
                div { title: "MdRepeat",
                    Icon { icon: MdRepeat, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRepeatOn") {
                div { title: "MdRepeatOn",
                    Icon { icon: MdRepeatOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRepeatOne") {
                div { title: "MdRepeatOne",
                    Icon { icon: MdRepeatOne, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRepeatOneOn") {
                div { title: "MdRepeatOneOn",
                    Icon { icon: MdRepeatOneOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReplay") {
                div { title: "MdReplay",
                    Icon { icon: MdReplay, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReplay5") {
                div { title: "MdReplay5",
                    Icon { icon: MdReplay5, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReplay10") {
                div { title: "MdReplay10",
                    Icon { icon: MdReplay10, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReplay30") {
                div { title: "MdReplay30",
                    Icon { icon: MdReplay30, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReplayCircleFilled") {
                div { title: "MdReplayCircleFilled",
                    Icon { icon: MdReplayCircleFilled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReply") {
                div { title: "MdReply",
                    Icon { icon: MdReply, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReplyAll") {
                div { title: "MdReplyAll",
                    Icon { icon: MdReplyAll, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReport") {
                div { title: "MdReport",
                    Icon { icon: MdReport, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReportOff") {
                div { title: "MdReportOff",
                    Icon { icon: MdReportOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdReportProblem") {
                div { title: "MdReportProblem",
                    Icon { icon: MdReportProblem, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRequestPage") {
                div { title: "MdRequestPage",
                    Icon { icon: MdRequestPage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRequestQuote") {
                div { title: "MdRequestQuote",
                    Icon { icon: MdRequestQuote, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdResetTv") {
                div { title: "MdResetTv",
                    Icon { icon: MdResetTv, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRestaurant") {
                div { title: "MdRestaurant",
                    Icon { icon: MdRestaurant, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRestaurantMenu") {
                div { title: "MdRestaurantMenu",
                    Icon { icon: MdRestaurantMenu, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRestore") {
                div { title: "MdRestore",
                    Icon { icon: MdRestore, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRestoreFromTrash") {
                div { title: "MdRestoreFromTrash",
                    Icon { icon: MdRestoreFromTrash, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRestorePage") {
                div { title: "MdRestorePage",
                    Icon { icon: MdRestorePage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRiceBowl") {
                div { title: "MdRiceBowl",
                    Icon { icon: MdRiceBowl, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRingVolume") {
                div { title: "MdRingVolume",
                    Icon { icon: MdRingVolume, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRoofing") {
                div { title: "MdRoofing",
                    Icon { icon: MdRoofing, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRoom") {
                div { title: "MdRoom",
                    Icon { icon: MdRoom, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRoomPreferences") {
                div { title: "MdRoomPreferences",
                    Icon { icon: MdRoomPreferences, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRoomService") {
                div { title: "MdRoomService",
                    Icon { icon: MdRoomService, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRotate90DegreesCcw") {
                div { title: "MdRotate90DegreesCcw",
                    Icon { icon: MdRotate90DegreesCcw, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRotateLeft") {
                div { title: "MdRotateLeft",
                    Icon { icon: MdRotateLeft, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRotateRight") {
                div { title: "MdRotateRight",
                    Icon { icon: MdRotateRight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRoundedCorner") {
                div { title: "MdRoundedCorner",
                    Icon { icon: MdRoundedCorner, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRouter") {
                div { title: "MdRouter",
                    Icon { icon: MdRouter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRowing") {
                div { title: "MdRowing",
                    Icon { icon: MdRowing, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRssFeed") {
                div { title: "MdRssFeed",
                    Icon { icon: MdRssFeed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRtt") {
                div { title: "MdRtt",
                    Icon { icon: MdRtt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRule") {
                div { title: "MdRule",
                    Icon { icon: MdRule, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRuleFolder") {
                div { title: "MdRuleFolder",
                    Icon { icon: MdRuleFolder, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRunCircle") {
                div { title: "MdRunCircle",
                    Icon { icon: MdRunCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdRvHookup") {
                div { title: "MdRvHookup",
                    Icon { icon: MdRvHookup, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSanitizer") {
                div { title: "MdSanitizer",
                    Icon { icon: MdSanitizer, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSatellite") {
                div { title: "MdSatellite",
                    Icon { icon: MdSatellite, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSave") {
                div { title: "MdSave",
                    Icon { icon: MdSave, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSaveAlt") {
                div { title: "MdSaveAlt",
                    Icon { icon: MdSaveAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSavedSearch") {
                div { title: "MdSavedSearch",
                    Icon { icon: MdSavedSearch, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScanner") {
                div { title: "MdScanner",
                    Icon { icon: MdScanner, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScatterPlot") {
                div { title: "MdScatterPlot",
                    Icon { icon: MdScatterPlot, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSchedule") {
                div { title: "MdSchedule",
                    Icon { icon: MdSchedule, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScheduleSend") {
                div { title: "MdScheduleSend",
                    Icon { icon: MdScheduleSend, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSchool") {
                div { title: "MdSchool",
                    Icon { icon: MdSchool, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScience") {
                div { title: "MdScience",
                    Icon { icon: MdScience, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScore") {
                div { title: "MdScore",
                    Icon { icon: MdScore, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScreenLockLandscape") {
                div { title: "MdScreenLockLandscape",
                    Icon { icon: MdScreenLockLandscape, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScreenLockPortrait") {
                div { title: "MdScreenLockPortrait",
                    Icon { icon: MdScreenLockPortrait, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScreenLockRotation") {
                div { title: "MdScreenLockRotation",
                    Icon { icon: MdScreenLockRotation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScreenRotation") {
                div { title: "MdScreenRotation",
                    Icon { icon: MdScreenRotation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScreenSearchDesktop") {
                div { title: "MdScreenSearchDesktop",
                    Icon { icon: MdScreenSearchDesktop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdScreenShare") {
                div { title: "MdScreenShare",
                    Icon { icon: MdScreenShare, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSd") {
                div { title: "MdSd",
                    Icon { icon: MdSd, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSdCard") {
                div { title: "MdSdCard",
                    Icon { icon: MdSdCard, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSdStorage") {
                div { title: "MdSdStorage",
                    Icon { icon: MdSdStorage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSearch") {
                div { title: "MdSearch",
                    Icon { icon: MdSearch, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSearchOff") {
                div { title: "MdSearchOff",
                    Icon { icon: MdSearchOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSecurity") {
                div { title: "MdSecurity",
                    Icon { icon: MdSecurity, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSegment") {
                div { title: "MdSegment",
                    Icon { icon: MdSegment, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSelectAll") {
                div { title: "MdSelectAll",
                    Icon { icon: MdSelectAll, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSelfImprovement") {
                div { title: "MdSelfImprovement",
                    Icon { icon: MdSelfImprovement, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSend") {
                div { title: "MdSend",
                    Icon { icon: MdSend, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSendAndArchive") {
                div { title: "MdSendAndArchive",
                    Icon { icon: MdSendAndArchive, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSendToMobile") {
                div { title: "MdSendToMobile",
                    Icon { icon: MdSendToMobile, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSensorDoor") {
                div { title: "MdSensorDoor",
                    Icon { icon: MdSensorDoor, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSensorWindow") {
                div { title: "MdSensorWindow",
                    Icon { icon: MdSensorWindow, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSentimentDissatisfied") {
                div { title: "MdSentimentDissatisfied",
                    Icon {
                        icon: MdSentimentDissatisfied,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSentimentNeutral") {
                div { title: "MdSentimentNeutral",
                    Icon { icon: MdSentimentNeutral, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSentimentSatisfied") {
                div { title: "MdSentimentSatisfied",
                    Icon { icon: MdSentimentSatisfied, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSentimentSatisfiedAlt") {
                div { title: "MdSentimentSatisfiedAlt",
                    Icon {
                        icon: MdSentimentSatisfiedAlt,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSentimentVeryDissatisfied") {
                div { title: "MdSentimentVeryDissatisfied",
                    Icon {
                        icon: MdSentimentVeryDissatisfied,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSentimentVerySatisfied") {
                div { title: "MdSentimentVerySatisfied",
                    Icon {
                        icon: MdSentimentVerySatisfied,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSetMeal") {
                div { title: "MdSetMeal",
                    Icon { icon: MdSetMeal, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettings") {
                div { title: "MdSettings",
                    Icon { icon: MdSettings, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsApplications") {
                div { title: "MdSettingsApplications",
                    Icon { icon: MdSettingsApplications, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsBackupRestore") {
                div { title: "MdSettingsBackupRestore",
                    Icon {
                        icon: MdSettingsBackupRestore,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSettingsBluetooth") {
                div { title: "MdSettingsBluetooth",
                    Icon { icon: MdSettingsBluetooth, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsBrightness") {
                div { title: "MdSettingsBrightness",
                    Icon { icon: MdSettingsBrightness, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsCell") {
                div { title: "MdSettingsCell",
                    Icon { icon: MdSettingsCell, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsEthernet") {
                div { title: "MdSettingsEthernet",
                    Icon { icon: MdSettingsEthernet, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsInputAntenna") {
                div { title: "MdSettingsInputAntenna",
                    Icon { icon: MdSettingsInputAntenna, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsInputComponent") {
                div { title: "MdSettingsInputComponent",
                    Icon {
                        icon: MdSettingsInputComponent,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSettingsInputComposite") {
                div { title: "MdSettingsInputComposite",
                    Icon {
                        icon: MdSettingsInputComposite,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSettingsInputHdmi") {
                div { title: "MdSettingsInputHdmi",
                    Icon { icon: MdSettingsInputHdmi, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsInputSvideo") {
                div { title: "MdSettingsInputSvideo",
                    Icon { icon: MdSettingsInputSvideo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsOverscan") {
                div { title: "MdSettingsOverscan",
                    Icon { icon: MdSettingsOverscan, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsPhone") {
                div { title: "MdSettingsPhone",
                    Icon { icon: MdSettingsPhone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsPower") {
                div { title: "MdSettingsPower",
                    Icon { icon: MdSettingsPower, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotM(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdSettingsRemote") {
                div { title: "MdSettingsRemote",
                    Icon { icon: MdSettingsRemote, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSettingsSystemDaydream") {
                div { title: "MdSettingsSystemDaydream",
                    Icon {
                        icon: MdSettingsSystemDaydream,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSettingsVoice") {
                div { title: "MdSettingsVoice",
                    Icon { icon: MdSettingsVoice, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShare") {
                div { title: "MdShare",
                    Icon { icon: MdShare, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShield") {
                div { title: "MdShield",
                    Icon { icon: MdShield, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShop") {
                div { title: "MdShop",
                    Icon { icon: MdShop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShoppingBag") {
                div { title: "MdShoppingBag",
                    Icon { icon: MdShoppingBag, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShoppingBasket") {
                div { title: "MdShoppingBasket",
                    Icon { icon: MdShoppingBasket, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShoppingCart") {
                div { title: "MdShoppingCart",
                    Icon { icon: MdShoppingCart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShopTwo") {
                div { title: "MdShopTwo",
                    Icon { icon: MdShopTwo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShortText") {
                div { title: "MdShortText",
                    Icon { icon: MdShortText, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShowChart") {
                div { title: "MdShowChart",
                    Icon { icon: MdShowChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShuffle") {
                div { title: "MdShuffle",
                    Icon { icon: MdShuffle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShuffleOn") {
                div { title: "MdShuffleOn",
                    Icon { icon: MdShuffleOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdShutterSpeed") {
                div { title: "MdShutterSpeed",
                    Icon { icon: MdShutterSpeed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSick") {
                div { title: "MdSick",
                    Icon { icon: MdSick, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSignalCellular0Bar") {
                div { title: "MdSignalCellular0Bar",
                    Icon { icon: MdSignalCellular0Bar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSignalCellular4Bar") {
                div { title: "MdSignalCellular4Bar",
                    Icon { icon: MdSignalCellular4Bar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSignalCellularAlt") {
                div { title: "MdSignalCellularAlt",
                    Icon { icon: MdSignalCellularAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSignalCellularConnectedNoInternet4Bar") {
                div { title: "MdSignalCellularConnectedNoInternet4Bar",
                    Icon {
                        icon: MdSignalCellularConnectedNoInternet4Bar,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSignalCellularNoSim") {
                div { title: "MdSignalCellularNoSim",
                    Icon { icon: MdSignalCellularNoSim, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSignalCellularNull") {
                div { title: "MdSignalCellularNull",
                    Icon { icon: MdSignalCellularNull, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSignalCellularOff") {
                div { title: "MdSignalCellularOff",
                    Icon { icon: MdSignalCellularOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSignalWifi0Bar") {
                div { title: "MdSignalWifi0Bar",
                    Icon { icon: MdSignalWifi0Bar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSignalWifi4Bar") {
                div { title: "MdSignalWifi4Bar",
                    Icon { icon: MdSignalWifi4Bar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSignalWifi4BarLock") {
                div { title: "MdSignalWifi4BarLock",
                    Icon { icon: MdSignalWifi4BarLock, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSignalWifiOff") {
                div { title: "MdSignalWifiOff",
                    Icon { icon: MdSignalWifiOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSimCard") {
                div { title: "MdSimCard",
                    Icon { icon: MdSimCard, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSimCardAlert") {
                div { title: "MdSimCardAlert",
                    Icon { icon: MdSimCardAlert, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSingleBed") {
                div { title: "MdSingleBed",
                    Icon { icon: MdSingleBed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSkipNext") {
                div { title: "MdSkipNext",
                    Icon { icon: MdSkipNext, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSkipPrevious") {
                div { title: "MdSkipPrevious",
                    Icon { icon: MdSkipPrevious, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSlideshow") {
                div { title: "MdSlideshow",
                    Icon { icon: MdSlideshow, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSlowMotionVideo") {
                div { title: "MdSlowMotionVideo",
                    Icon { icon: MdSlowMotionVideo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSmartButton") {
                div { title: "MdSmartButton",
                    Icon { icon: MdSmartButton, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSmartphone") {
                div { title: "MdSmartphone",
                    Icon { icon: MdSmartphone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSmokeFree") {
                div { title: "MdSmokeFree",
                    Icon { icon: MdSmokeFree, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSmokingRooms") {
                div { title: "MdSmokingRooms",
                    Icon { icon: MdSmokingRooms, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSms") {
                div { title: "MdSms",
                    Icon { icon: MdSms, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSmsFailed") {
                div { title: "MdSmsFailed",
                    Icon { icon: MdSmsFailed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSnippetFolder") {
                div { title: "MdSnippetFolder",
                    Icon { icon: MdSnippetFolder, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSnooze") {
                div { title: "MdSnooze",
                    Icon { icon: MdSnooze, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSoap") {
                div { title: "MdSoap",
                    Icon { icon: MdSoap, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSort") {
                div { title: "MdSort",
                    Icon { icon: MdSort, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSortByAlpha") {
                div { title: "MdSortByAlpha",
                    Icon { icon: MdSortByAlpha, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSource") {
                div { title: "MdSource",
                    Icon { icon: MdSource, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSouth") {
                div { title: "MdSouth",
                    Icon { icon: MdSouth, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSouthEast") {
                div { title: "MdSouthEast",
                    Icon { icon: MdSouthEast, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSouthWest") {
                div { title: "MdSouthWest",
                    Icon { icon: MdSouthWest, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSpa") {
                div { title: "MdSpa",
                    Icon { icon: MdSpa, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSpaceBar") {
                div { title: "MdSpaceBar",
                    Icon { icon: MdSpaceBar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSpeaker") {
                div { title: "MdSpeaker",
                    Icon { icon: MdSpeaker, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSpeakerGroup") {
                div { title: "MdSpeakerGroup",
                    Icon { icon: MdSpeakerGroup, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSpeakerNotes") {
                div { title: "MdSpeakerNotes",
                    Icon { icon: MdSpeakerNotes, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSpeakerNotesOff") {
                div { title: "MdSpeakerNotesOff",
                    Icon { icon: MdSpeakerNotesOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSpeakerPhone") {
                div { title: "MdSpeakerPhone",
                    Icon { icon: MdSpeakerPhone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSpeed") {
                div { title: "MdSpeed",
                    Icon { icon: MdSpeed, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSpellcheck") {
                div { title: "MdSpellcheck",
                    Icon { icon: MdSpellcheck, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSports") {
                div { title: "MdSports",
                    Icon { icon: MdSports, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsBar") {
                div { title: "MdSportsBar",
                    Icon { icon: MdSportsBar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsBaseball") {
                div { title: "MdSportsBaseball",
                    Icon { icon: MdSportsBaseball, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsBasketball") {
                div { title: "MdSportsBasketball",
                    Icon { icon: MdSportsBasketball, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsCricket") {
                div { title: "MdSportsCricket",
                    Icon { icon: MdSportsCricket, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsEsports") {
                div { title: "MdSportsEsports",
                    Icon { icon: MdSportsEsports, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsFootball") {
                div { title: "MdSportsFootball",
                    Icon { icon: MdSportsFootball, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsGolf") {
                div { title: "MdSportsGolf",
                    Icon { icon: MdSportsGolf, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsHandball") {
                div { title: "MdSportsHandball",
                    Icon { icon: MdSportsHandball, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsHockey") {
                div { title: "MdSportsHockey",
                    Icon { icon: MdSportsHockey, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsKabaddi") {
                div { title: "MdSportsKabaddi",
                    Icon { icon: MdSportsKabaddi, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsMma") {
                div { title: "MdSportsMma",
                    Icon { icon: MdSportsMma, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsMotorsports") {
                div { title: "MdSportsMotorsports",
                    Icon { icon: MdSportsMotorsports, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsRugby") {
                div { title: "MdSportsRugby",
                    Icon { icon: MdSportsRugby, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsSoccer") {
                div { title: "MdSportsSoccer",
                    Icon { icon: MdSportsSoccer, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsTennis") {
                div { title: "MdSportsTennis",
                    Icon { icon: MdSportsTennis, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSportsVolleyball") {
                div { title: "MdSportsVolleyball",
                    Icon { icon: MdSportsVolleyball, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSquareFoot") {
                div { title: "MdSquareFoot",
                    Icon { icon: MdSquareFoot, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStackedBarChart") {
                div { title: "MdStackedBarChart",
                    Icon { icon: MdStackedBarChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStackedLineChart") {
                div { title: "MdStackedLineChart",
                    Icon { icon: MdStackedLineChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStairs") {
                div { title: "MdStairs",
                    Icon { icon: MdStairs, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStar") {
                div { title: "MdStar",
                    Icon { icon: MdStar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStarBorder") {
                div { title: "MdStarBorder",
                    Icon { icon: MdStarBorder, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStarHalf") {
                div { title: "MdStarHalf",
                    Icon { icon: MdStarHalf, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStarOutline") {
                div { title: "MdStarOutline",
                    Icon { icon: MdStarOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStarRate") {
                div { title: "MdStarRate",
                    Icon { icon: MdStarRate, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStars") {
                div { title: "MdStars",
                    Icon { icon: MdStars, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStayCurrentLandscape") {
                div { title: "MdStayCurrentLandscape",
                    Icon { icon: MdStayCurrentLandscape, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStayCurrentPortrait") {
                div { title: "MdStayCurrentPortrait",
                    Icon { icon: MdStayCurrentPortrait, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStayPrimaryLandscape") {
                div { title: "MdStayPrimaryLandscape",
                    Icon { icon: MdStayPrimaryLandscape, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStayPrimaryPortrait") {
                div { title: "MdStayPrimaryPortrait",
                    Icon { icon: MdStayPrimaryPortrait, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStickyNote2") {
                div { title: "MdStickyNote2",
                    Icon { icon: MdStickyNote2, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStop") {
                div { title: "MdStop",
                    Icon { icon: MdStop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStopCircle") {
                div { title: "MdStopCircle",
                    Icon { icon: MdStopCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStopScreenShare") {
                div { title: "MdStopScreenShare",
                    Icon { icon: MdStopScreenShare, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStorage") {
                div { title: "MdStorage",
                    Icon { icon: MdStorage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStore") {
                div { title: "MdStore",
                    Icon { icon: MdStore, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStorefront") {
                div { title: "MdStorefront",
                    Icon { icon: MdStorefront, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStoreMallDirectory") {
                div { title: "MdStoreMallDirectory",
                    Icon { icon: MdStoreMallDirectory, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStraighten") {
                div { title: "MdStraighten",
                    Icon { icon: MdStraighten, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStream") {
                div { title: "MdStream",
                    Icon { icon: MdStream, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStreetview") {
                div { title: "MdStreetview",
                    Icon { icon: MdStreetview, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotN(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdStrikethroughS") {
                div { title: "MdStrikethroughS",
                    Icon { icon: MdStrikethroughS, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStroller") {
                div { title: "MdStroller",
                    Icon { icon: MdStroller, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdStyle") {
                div { title: "MdStyle",
                    Icon { icon: MdStyle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSubdirectoryArrowLeft") {
                div { title: "MdSubdirectoryArrowLeft",
                    Icon {
                        icon: MdSubdirectoryArrowLeft,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSubdirectoryArrowRight") {
                div { title: "MdSubdirectoryArrowRight",
                    Icon {
                        icon: MdSubdirectoryArrowRight,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdSubject") {
                div { title: "MdSubject",
                    Icon { icon: MdSubject, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSubscript") {
                div { title: "MdSubscript",
                    Icon { icon: MdSubscript, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSubscriptions") {
                div { title: "MdSubscriptions",
                    Icon { icon: MdSubscriptions, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSubtitles") {
                div { title: "MdSubtitles",
                    Icon { icon: MdSubtitles, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSubtitlesOff") {
                div { title: "MdSubtitlesOff",
                    Icon { icon: MdSubtitlesOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSubway") {
                div { title: "MdSubway",
                    Icon { icon: MdSubway, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSuperscript") {
                div { title: "MdSuperscript",
                    Icon { icon: MdSuperscript, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSupervisedUserCircle") {
                div { title: "MdSupervisedUserCircle",
                    Icon { icon: MdSupervisedUserCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSupervisorAccount") {
                div { title: "MdSupervisorAccount",
                    Icon { icon: MdSupervisorAccount, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSupport") {
                div { title: "MdSupport",
                    Icon { icon: MdSupport, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSupportAgent") {
                div { title: "MdSupportAgent",
                    Icon { icon: MdSupportAgent, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSurroundSound") {
                div { title: "MdSurroundSound",
                    Icon { icon: MdSurroundSound, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwapCalls") {
                div { title: "MdSwapCalls",
                    Icon { icon: MdSwapCalls, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwapHoriz") {
                div { title: "MdSwapHoriz",
                    Icon { icon: MdSwapHoriz, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwapHorizontalCircle") {
                div { title: "MdSwapHorizontalCircle",
                    Icon { icon: MdSwapHorizontalCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwapVert") {
                div { title: "MdSwapVert",
                    Icon { icon: MdSwapVert, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwapVerticalCircle") {
                div { title: "MdSwapVerticalCircle",
                    Icon { icon: MdSwapVerticalCircle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwipe") {
                div { title: "MdSwipe",
                    Icon { icon: MdSwipe, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwitchAccount") {
                div { title: "MdSwitchAccount",
                    Icon { icon: MdSwitchAccount, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwitchCamera") {
                div { title: "MdSwitchCamera",
                    Icon { icon: MdSwitchCamera, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwitchLeft") {
                div { title: "MdSwitchLeft",
                    Icon { icon: MdSwitchLeft, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwitchRight") {
                div { title: "MdSwitchRight",
                    Icon { icon: MdSwitchRight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSwitchVideo") {
                div { title: "MdSwitchVideo",
                    Icon { icon: MdSwitchVideo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSync") {
                div { title: "MdSync",
                    Icon { icon: MdSync, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSyncAlt") {
                div { title: "MdSyncAlt",
                    Icon { icon: MdSyncAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSyncDisabled") {
                div { title: "MdSyncDisabled",
                    Icon { icon: MdSyncDisabled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSyncProblem") {
                div { title: "MdSyncProblem",
                    Icon { icon: MdSyncProblem, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSystemUpdate") {
                div { title: "MdSystemUpdate",
                    Icon { icon: MdSystemUpdate, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdSystemUpdateAlt") {
                div { title: "MdSystemUpdateAlt",
                    Icon { icon: MdSystemUpdateAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTab") {
                div { title: "MdTab",
                    Icon { icon: MdTab, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTableChart") {
                div { title: "MdTableChart",
                    Icon { icon: MdTableChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTableRows") {
                div { title: "MdTableRows",
                    Icon { icon: MdTableRows, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTablet") {
                div { title: "MdTablet",
                    Icon { icon: MdTablet, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTabletAndroid") {
                div { title: "MdTabletAndroid",
                    Icon { icon: MdTabletAndroid, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTabletMac") {
                div { title: "MdTabletMac",
                    Icon { icon: MdTabletMac, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTableView") {
                div { title: "MdTableView",
                    Icon { icon: MdTableView, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTabUnselected") {
                div { title: "MdTabUnselected",
                    Icon { icon: MdTabUnselected, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTag") {
                div { title: "MdTag",
                    Icon { icon: MdTag, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTagFaces") {
                div { title: "MdTagFaces",
                    Icon { icon: MdTagFaces, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTakeoutDining") {
                div { title: "MdTakeoutDining",
                    Icon { icon: MdTakeoutDining, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTapAndPlay") {
                div { title: "MdTapAndPlay",
                    Icon { icon: MdTapAndPlay, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTapas") {
                div { title: "MdTapas",
                    Icon { icon: MdTapas, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTaxiAlert") {
                div { title: "MdTaxiAlert",
                    Icon { icon: MdTaxiAlert, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTerrain") {
                div { title: "MdTerrain",
                    Icon { icon: MdTerrain, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTextFields") {
                div { title: "MdTextFields",
                    Icon { icon: MdTextFields, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTextFormat") {
                div { title: "MdTextFormat",
                    Icon { icon: MdTextFormat, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTextRotateUp") {
                div { title: "MdTextRotateUp",
                    Icon { icon: MdTextRotateUp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTextRotateVertical") {
                div { title: "MdTextRotateVertical",
                    Icon { icon: MdTextRotateVertical, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTextRotationAngledown") {
                div { title: "MdTextRotationAngledown",
                    Icon {
                        icon: MdTextRotationAngledown,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdTextRotationAngleup") {
                div { title: "MdTextRotationAngleup",
                    Icon { icon: MdTextRotationAngleup, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTextRotationDown") {
                div { title: "MdTextRotationDown",
                    Icon { icon: MdTextRotationDown, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTextRotationNone") {
                div { title: "MdTextRotationNone",
                    Icon { icon: MdTextRotationNone, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTextsms") {
                div { title: "MdTextsms",
                    Icon { icon: MdTextsms, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTextSnippet") {
                div { title: "MdTextSnippet",
                    Icon { icon: MdTextSnippet, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTexture") {
                div { title: "MdTexture",
                    Icon { icon: MdTexture, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTheaterComedy") {
                div { title: "MdTheaterComedy",
                    Icon { icon: MdTheaterComedy, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTheaters") {
                div { title: "MdTheaters",
                    Icon { icon: MdTheaters, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdThumbDown") {
                div { title: "MdThumbDown",
                    Icon { icon: MdThumbDown, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdThumbDownAlt") {
                div { title: "MdThumbDownAlt",
                    Icon { icon: MdThumbDownAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdThumbDownOffAlt") {
                div { title: "MdThumbDownOffAlt",
                    Icon { icon: MdThumbDownOffAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdThumbsUpDown") {
                div { title: "MdThumbsUpDown",
                    Icon { icon: MdThumbsUpDown, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdThumbUp") {
                div { title: "MdThumbUp",
                    Icon { icon: MdThumbUp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdThumbUpAlt") {
                div { title: "MdThumbUpAlt",
                    Icon { icon: MdThumbUpAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdThumbUpOffAlt") {
                div { title: "MdThumbUpOffAlt",
                    Icon { icon: MdThumbUpOffAlt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTimelapse") {
                div { title: "MdTimelapse",
                    Icon { icon: MdTimelapse, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTimeline") {
                div { title: "MdTimeline",
                    Icon { icon: MdTimeline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTimer") {
                div { title: "MdTimer",
                    Icon { icon: MdTimer, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTimer3") {
                div { title: "MdTimer3",
                    Icon { icon: MdTimer3, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTimer10") {
                div { title: "MdTimer10",
                    Icon { icon: MdTimer10, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTimerOff") {
                div { title: "MdTimerOff",
                    Icon { icon: MdTimerOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTimeToLeave") {
                div { title: "MdTimeToLeave",
                    Icon { icon: MdTimeToLeave, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTitle") {
                div { title: "MdTitle",
                    Icon { icon: MdTitle, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdToc") {
                div { title: "MdToc",
                    Icon { icon: MdToc, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdToday") {
                div { title: "MdToday",
                    Icon { icon: MdToday, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdToggleOff") {
                div { title: "MdToggleOff",
                    Icon { icon: MdToggleOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdToggleOn") {
                div { title: "MdToggleOn",
                    Icon { icon: MdToggleOn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdToll") {
                div { title: "MdToll",
                    Icon { icon: MdToll, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTonality") {
                div { title: "MdTonality",
                    Icon { icon: MdTonality, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTopic") {
                div { title: "MdTopic",
                    Icon { icon: MdTopic, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTouchApp") {
                div { title: "MdTouchApp",
                    Icon { icon: MdTouchApp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTour") {
                div { title: "MdTour",
                    Icon { icon: MdTour, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdToys") {
                div { title: "MdToys",
                    Icon { icon: MdToys, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTrackChanges") {
                div { title: "MdTrackChanges",
                    Icon { icon: MdTrackChanges, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTraffic") {
                div { title: "MdTraffic",
                    Icon { icon: MdTraffic, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTrain") {
                div { title: "MdTrain",
                    Icon { icon: MdTrain, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTram") {
                div { title: "MdTram",
                    Icon { icon: MdTram, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTransferWithinAStation") {
                div { title: "MdTransferWithinAStation",
                    Icon {
                        icon: MdTransferWithinAStation,
                        height: 48,
                        width: 48,
                    }
                }
            }
            if found_icons.read().contains("MdTransform") {
                div { title: "MdTransform",
                    Icon { icon: MdTransform, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTransitEnterexit") {
                div { title: "MdTransitEnterexit",
                    Icon { icon: MdTransitEnterexit, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTranslate") {
                div { title: "MdTranslate",
                    Icon { icon: MdTranslate, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTrendingDown") {
                div { title: "MdTrendingDown",
                    Icon { icon: MdTrendingDown, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTrendingFlat") {
                div { title: "MdTrendingFlat",
                    Icon { icon: MdTrendingFlat, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTrendingUp") {
                div { title: "MdTrendingUp",
                    Icon { icon: MdTrendingUp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTripOrigin") {
                div { title: "MdTripOrigin",
                    Icon { icon: MdTripOrigin, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTty") {
                div { title: "MdTty",
                    Icon { icon: MdTty, height: 48, width: 48 }
                }
            }
        }
    }
}

#[component]
fn LotO(found_icons: ReadSignal<HashSet<&'static str>>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            if found_icons.read().contains("MdTune") {
                div { title: "MdTune",
                    Icon { icon: MdTune, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTurnedIn") {
                div { title: "MdTurnedIn",
                    Icon { icon: MdTurnedIn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTurnedInNot") {
                div { title: "MdTurnedInNot",
                    Icon { icon: MdTurnedInNot, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTv") {
                div { title: "MdTv",
                    Icon { icon: MdTv, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTvOff") {
                div { title: "MdTvOff",
                    Icon { icon: MdTvOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdTwoWheeler") {
                div { title: "MdTwoWheeler",
                    Icon { icon: MdTwoWheeler, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUmbrella") {
                div { title: "MdUmbrella",
                    Icon { icon: MdUmbrella, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUnarchive") {
                div { title: "MdUnarchive",
                    Icon { icon: MdUnarchive, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUndo") {
                div { title: "MdUndo",
                    Icon { icon: MdUndo, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUnfoldLess") {
                div { title: "MdUnfoldLess",
                    Icon { icon: MdUnfoldLess, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUnfoldMore") {
                div { title: "MdUnfoldMore",
                    Icon { icon: MdUnfoldMore, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUnpublished") {
                div { title: "MdUnpublished",
                    Icon { icon: MdUnpublished, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUnsubscribe") {
                div { title: "MdUnsubscribe",
                    Icon { icon: MdUnsubscribe, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUpdate") {
                div { title: "MdUpdate",
                    Icon { icon: MdUpdate, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUpgrade") {
                div { title: "MdUpgrade",
                    Icon { icon: MdUpgrade, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUploadFile") {
                div { title: "MdUploadFile",
                    Icon { icon: MdUploadFile, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdUsb") {
                div { title: "MdUsb",
                    Icon { icon: MdUsb, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVerified") {
                div { title: "MdVerified",
                    Icon { icon: MdVerified, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVerifiedUser") {
                div { title: "MdVerifiedUser",
                    Icon { icon: MdVerifiedUser, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVerticalAlignBottom") {
                div { title: "MdVerticalAlignBottom",
                    Icon { icon: MdVerticalAlignBottom, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVerticalAlignCenter") {
                div { title: "MdVerticalAlignCenter",
                    Icon { icon: MdVerticalAlignCenter, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVerticalAlignTop") {
                div { title: "MdVerticalAlignTop",
                    Icon { icon: MdVerticalAlignTop, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVerticalSplit") {
                div { title: "MdVerticalSplit",
                    Icon { icon: MdVerticalSplit, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVibration") {
                div { title: "MdVibration",
                    Icon { icon: MdVibration, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVideoCall") {
                div { title: "MdVideoCall",
                    Icon { icon: MdVideoCall, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVideocam") {
                div { title: "MdVideocam",
                    Icon { icon: MdVideocam, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVideocamOff") {
                div { title: "MdVideocamOff",
                    Icon { icon: MdVideocamOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVideogameAsset") {
                div { title: "MdVideogameAsset",
                    Icon { icon: MdVideogameAsset, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVideoLabel") {
                div { title: "MdVideoLabel",
                    Icon { icon: MdVideoLabel, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVideoLibrary") {
                div { title: "MdVideoLibrary",
                    Icon { icon: MdVideoLibrary, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVideoSettings") {
                div { title: "MdVideoSettings",
                    Icon { icon: MdVideoSettings, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewAgenda") {
                div { title: "MdViewAgenda",
                    Icon { icon: MdViewAgenda, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewArray") {
                div { title: "MdViewArray",
                    Icon { icon: MdViewArray, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewCarousel") {
                div { title: "MdViewCarousel",
                    Icon { icon: MdViewCarousel, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewColumn") {
                div { title: "MdViewColumn",
                    Icon { icon: MdViewColumn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewComfy") {
                div { title: "MdViewComfy",
                    Icon { icon: MdViewComfy, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewCompact") {
                div { title: "MdViewCompact",
                    Icon { icon: MdViewCompact, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewDay") {
                div { title: "MdViewDay",
                    Icon { icon: MdViewDay, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewHeadline") {
                div { title: "MdViewHeadline",
                    Icon { icon: MdViewHeadline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewInAr") {
                div { title: "MdViewInAr",
                    Icon { icon: MdViewInAr, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewList") {
                div { title: "MdViewList",
                    Icon { icon: MdViewList, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewModule") {
                div { title: "MdViewModule",
                    Icon { icon: MdViewModule, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewQuilt") {
                div { title: "MdViewQuilt",
                    Icon { icon: MdViewQuilt, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewSidebar") {
                div { title: "MdViewSidebar",
                    Icon { icon: MdViewSidebar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewStream") {
                div { title: "MdViewStream",
                    Icon { icon: MdViewStream, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdViewWeek") {
                div { title: "MdViewWeek",
                    Icon { icon: MdViewWeek, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVignette") {
                div { title: "MdVignette",
                    Icon { icon: MdVignette, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVisibility") {
                div { title: "MdVisibility",
                    Icon { icon: MdVisibility, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVisibilityOff") {
                div { title: "MdVisibilityOff",
                    Icon { icon: MdVisibilityOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVoiceChat") {
                div { title: "MdVoiceChat",
                    Icon { icon: MdVoiceChat, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVoicemail") {
                div { title: "MdVoicemail",
                    Icon { icon: MdVoicemail, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVoiceOverOff") {
                div { title: "MdVoiceOverOff",
                    Icon { icon: MdVoiceOverOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVolumeDown") {
                div { title: "MdVolumeDown",
                    Icon { icon: MdVolumeDown, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVolumeMute") {
                div { title: "MdVolumeMute",
                    Icon { icon: MdVolumeMute, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVolumeOff") {
                div { title: "MdVolumeOff",
                    Icon { icon: MdVolumeOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVolumeUp") {
                div { title: "MdVolumeUp",
                    Icon { icon: MdVolumeUp, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVolunteerActivism") {
                div { title: "MdVolunteerActivism",
                    Icon { icon: MdVolunteerActivism, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVpnKey") {
                div { title: "MdVpnKey",
                    Icon { icon: MdVpnKey, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdVpnLock") {
                div { title: "MdVpnLock",
                    Icon { icon: MdVpnLock, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWallpaper") {
                div { title: "MdWallpaper",
                    Icon { icon: MdWallpaper, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWarning") {
                div { title: "MdWarning",
                    Icon { icon: MdWarning, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWash") {
                div { title: "MdWash",
                    Icon { icon: MdWash, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWatch") {
                div { title: "MdWatch",
                    Icon { icon: MdWatch, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWatchLater") {
                div { title: "MdWatchLater",
                    Icon { icon: MdWatchLater, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWaterDamage") {
                div { title: "MdWaterDamage",
                    Icon { icon: MdWaterDamage, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWaterfallChart") {
                div { title: "MdWaterfallChart",
                    Icon { icon: MdWaterfallChart, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWaves") {
                div { title: "MdWaves",
                    Icon { icon: MdWaves, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWbAuto") {
                div { title: "MdWbAuto",
                    Icon { icon: MdWbAuto, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWbCloudy") {
                div { title: "MdWbCloudy",
                    Icon { icon: MdWbCloudy, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWbIncandescent") {
                div { title: "MdWbIncandescent",
                    Icon { icon: MdWbIncandescent, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWbIridescent") {
                div { title: "MdWbIridescent",
                    Icon { icon: MdWbIridescent, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWbShade") {
                div { title: "MdWbShade",
                    Icon { icon: MdWbShade, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWbSunny") {
                div { title: "MdWbSunny",
                    Icon { icon: MdWbSunny, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWbTwighlight") {
                div { title: "MdWbTwighlight",
                    Icon { icon: MdWbTwighlight, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWc") {
                div { title: "MdWc",
                    Icon { icon: MdWc, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWeb") {
                div { title: "MdWeb",
                    Icon { icon: MdWeb, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWebAsset") {
                div { title: "MdWebAsset",
                    Icon { icon: MdWebAsset, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWeekend") {
                div { title: "MdWeekend",
                    Icon { icon: MdWeekend, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWest") {
                div { title: "MdWest",
                    Icon { icon: MdWest, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWhatshot") {
                div { title: "MdWhatshot",
                    Icon { icon: MdWhatshot, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWheelchairPickup") {
                div { title: "MdWheelchairPickup",
                    Icon { icon: MdWheelchairPickup, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWhereToVote") {
                div { title: "MdWhereToVote",
                    Icon { icon: MdWhereToVote, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWidgets") {
                div { title: "MdWidgets",
                    Icon { icon: MdWidgets, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWifi") {
                div { title: "MdWifi",
                    Icon { icon: MdWifi, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWifiCalling") {
                div { title: "MdWifiCalling",
                    Icon { icon: MdWifiCalling, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWifiLock") {
                div { title: "MdWifiLock",
                    Icon { icon: MdWifiLock, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWifiOff") {
                div { title: "MdWifiOff",
                    Icon { icon: MdWifiOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWifiProtectedSetup") {
                div { title: "MdWifiProtectedSetup",
                    Icon { icon: MdWifiProtectedSetup, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWifiTethering") {
                div { title: "MdWifiTethering",
                    Icon { icon: MdWifiTethering, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWineBar") {
                div { title: "MdWineBar",
                    Icon { icon: MdWineBar, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWork") {
                div { title: "MdWork",
                    Icon { icon: MdWork, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWorkOff") {
                div { title: "MdWorkOff",
                    Icon { icon: MdWorkOff, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWorkOutline") {
                div { title: "MdWorkOutline",
                    Icon { icon: MdWorkOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWorkspacesFilled") {
                div { title: "MdWorkspacesFilled",
                    Icon { icon: MdWorkspacesFilled, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWorkspacesOutline") {
                div { title: "MdWorkspacesOutline",
                    Icon { icon: MdWorkspacesOutline, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWrapText") {
                div { title: "MdWrapText",
                    Icon { icon: MdWrapText, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWrongLocation") {
                div { title: "MdWrongLocation",
                    Icon { icon: MdWrongLocation, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdWysiwyg") {
                div { title: "MdWysiwyg",
                    Icon { icon: MdWysiwyg, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdYoutubeSearchedFor") {
                div { title: "MdYoutubeSearchedFor",
                    Icon { icon: MdYoutubeSearchedFor, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdZoomIn") {
                div { title: "MdZoomIn",
                    Icon { icon: MdZoomIn, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdZoomOut") {
                div { title: "MdZoomOut",
                    Icon { icon: MdZoomOut, height: 48, width: 48 }
                }
            }
            if found_icons.read().contains("MdZoomOutMap") {
                div { title: "MdZoomOutMap",
                    Icon { icon: MdZoomOutMap, height: 48, width: 48 }
                }
            }
        }
    }
}
