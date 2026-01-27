<script lang="ts">
    import type { Component } from "svelte";

    // Import directly from lib paths for tree-shaking
    import TableIcon from "phosphor-svelte/lib/TableIcon";
    import PackageIcon from "phosphor-svelte/lib/PackageIcon";
    import UserIcon from "phosphor-svelte/lib/UserIcon";
    import UsersIcon from "phosphor-svelte/lib/UsersIcon";
    import ShoppingCartIcon from "phosphor-svelte/lib/ShoppingCartIcon";
    import TagIcon from "phosphor-svelte/lib/TagIcon";
    import FolderIcon from "phosphor-svelte/lib/FolderIcon";
    import FileIcon from "phosphor-svelte/lib/FileIcon";
    import ImageIcon from "phosphor-svelte/lib/ImageIcon";
    import LinkIcon from "phosphor-svelte/lib/LinkIcon";
    import GlobeIcon from "phosphor-svelte/lib/GlobeIcon";
    import CurrencyDollarIcon from "phosphor-svelte/lib/CurrencyDollarIcon";
    import CreditCardIcon from "phosphor-svelte/lib/CreditCardIcon";
    import TruckIcon from "phosphor-svelte/lib/TruckIcon";
    import MapPinIcon from "phosphor-svelte/lib/MapPinIcon";
    import CalendarIcon from "phosphor-svelte/lib/CalendarIcon";
    import ClockIcon from "phosphor-svelte/lib/ClockIcon";
    import HashIcon from "phosphor-svelte/lib/HashIcon";
    import StarIcon from "phosphor-svelte/lib/StarIcon";
    import HeartIcon from "phosphor-svelte/lib/HeartIcon";
    import BellIcon from "phosphor-svelte/lib/BellIcon";
    import GearIcon from "phosphor-svelte/lib/GearIcon";
    import MagnifyingGlassIcon from "phosphor-svelte/lib/MagnifyingGlassIcon";
    import PencilIcon from "phosphor-svelte/lib/PencilIcon";
    import TrashIcon from "phosphor-svelte/lib/TrashIcon";
    import PlusIcon from "phosphor-svelte/lib/PlusIcon";
    import CheckIcon from "phosphor-svelte/lib/CheckIcon";
    import XIcon from "phosphor-svelte/lib/XIcon";
    import WarningIcon from "phosphor-svelte/lib/WarningIcon";
    import InfoIcon from "phosphor-svelte/lib/InfoIcon";
    import QuestionIcon from "phosphor-svelte/lib/QuestionIcon";
    import LockIcon from "phosphor-svelte/lib/LockIcon";
    import KeyIcon from "phosphor-svelte/lib/KeyIcon";
    import EnvelopeIcon from "phosphor-svelte/lib/EnvelopeIcon";
    import ChatIcon from "phosphor-svelte/lib/ChatIcon";
    import HouseIcon from "phosphor-svelte/lib/HouseIcon";
    import BuildingsIcon from "phosphor-svelte/lib/BuildingsIcon";
    import StorefrontIcon from "phosphor-svelte/lib/StorefrontIcon";
    import ReceiptIcon from "phosphor-svelte/lib/ReceiptIcon";
    import ListIcon from "phosphor-svelte/lib/ListIcon";
    import GridFourIcon from "phosphor-svelte/lib/GridFourIcon";
    import StackIcon from "phosphor-svelte/lib/StackIcon";
    import DatabaseIcon from "phosphor-svelte/lib/DatabaseIcon";
    import CodeIcon from "phosphor-svelte/lib/CodeIcon";
    import TerminalIcon from "phosphor-svelte/lib/TerminalIcon";
    import BracketsCurlyIcon from "phosphor-svelte/lib/BracketsCurlyIcon";
    import FlagIcon from "phosphor-svelte/lib/FlagIcon";
    import BookmarkIcon from "phosphor-svelte/lib/BookmarkIcon";
    import ArchiveIcon from "phosphor-svelte/lib/ArchiveIcon";
    import CloudIcon from "phosphor-svelte/lib/CloudIcon";
    import DownloadIcon from "phosphor-svelte/lib/DownloadIcon";
    import UploadIcon from "phosphor-svelte/lib/UploadIcon";
    import ShareIcon from "phosphor-svelte/lib/ShareIcon";
    import EyeIcon from "phosphor-svelte/lib/EyeIcon";
    import EyeSlashIcon from "phosphor-svelte/lib/EyeSlashIcon";
    import CopyIcon from "phosphor-svelte/lib/CopyIcon";
    import ScissorsIcon from "phosphor-svelte/lib/ScissorsIcon";
    import PaletteIcon from "phosphor-svelte/lib/PaletteIcon";
    import SlidersIcon from "phosphor-svelte/lib/SlidersIcon";
    import ChartLineIcon from "phosphor-svelte/lib/ChartLineIcon";
    import ChartBarIcon from "phosphor-svelte/lib/ChartBarIcon";
    import PercentIcon from "phosphor-svelte/lib/PercentIcon";
    import CoinsIcon from "phosphor-svelte/lib/CoinsIcon";
    import WalletIcon from "phosphor-svelte/lib/WalletIcon";
    import BankIcon from "phosphor-svelte/lib/BankIcon";
    import HandCoinsIcon from "phosphor-svelte/lib/HandCoinsIcon";
    import ArrowsClockwiseIcon from "phosphor-svelte/lib/ArrowsClockwiseIcon";
    import ArrowRightIcon from "phosphor-svelte/lib/ArrowRightIcon";
    import ArrowLeftIcon from "phosphor-svelte/lib/ArrowLeftIcon";
    import CaretDownIcon from "phosphor-svelte/lib/CaretDownIcon";
    import CaretRightIcon from "phosphor-svelte/lib/CaretRightIcon";
    import DotsThreeIcon from "phosphor-svelte/lib/DotsThreeIcon";

    interface Props {
        name: string;
        class?: string;
        size?: number;
        fallback?: string;
    }

    let { name, class: className = "", size = 16, fallback = "table" }: Props = $props();

    // Map of icon names to components
    const iconMap: Record<string, Component<{ size?: number; class?: string }>> = {
        table: TableIcon,
        package: PackageIcon,
        user: UserIcon,
        users: UsersIcon,
        "shopping-cart": ShoppingCartIcon,
        cart: ShoppingCartIcon,
        tag: TagIcon,
        folder: FolderIcon,
        file: FileIcon,
        image: ImageIcon,
        link: LinkIcon,
        globe: GlobeIcon,
        "currency-dollar": CurrencyDollarIcon,
        dollar: CurrencyDollarIcon,
        money: CurrencyDollarIcon,
        "credit-card": CreditCardIcon,
        card: CreditCardIcon,
        truck: TruckIcon,
        shipping: TruckIcon,
        "map-pin": MapPinIcon,
        location: MapPinIcon,
        address: MapPinIcon,
        calendar: CalendarIcon,
        clock: ClockIcon,
        time: ClockIcon,
        hash: HashIcon,
        number: HashIcon,
        star: StarIcon,
        heart: HeartIcon,
        favorite: HeartIcon,
        bell: BellIcon,
        notification: BellIcon,
        gear: GearIcon,
        settings: GearIcon,
        "magnifying-glass": MagnifyingGlassIcon,
        search: MagnifyingGlassIcon,
        pencil: PencilIcon,
        edit: PencilIcon,
        trash: TrashIcon,
        delete: TrashIcon,
        plus: PlusIcon,
        add: PlusIcon,
        check: CheckIcon,
        x: XIcon,
        close: XIcon,
        warning: WarningIcon,
        info: InfoIcon,
        question: QuestionIcon,
        help: QuestionIcon,
        lock: LockIcon,
        key: KeyIcon,
        envelope: EnvelopeIcon,
        email: EnvelopeIcon,
        mail: EnvelopeIcon,
        chat: ChatIcon,
        message: ChatIcon,
        house: HouseIcon,
        home: HouseIcon,
        buildings: BuildingsIcon,
        storefront: StorefrontIcon,
        store: StorefrontIcon,
        shop: StorefrontIcon,
        receipt: ReceiptIcon,
        invoice: ReceiptIcon,
        order: ReceiptIcon,
        list: ListIcon,
        "grid-four": GridFourIcon,
        grid: GridFourIcon,
        layers: StackIcon,
        stack: StackIcon,
        database: DatabaseIcon,
        code: CodeIcon,
        terminal: TerminalIcon,
        "brackets-curly": BracketsCurlyIcon,
        json: BracketsCurlyIcon,
        flag: FlagIcon,
        bookmark: BookmarkIcon,
        archive: ArchiveIcon,
        cloud: CloudIcon,
        download: DownloadIcon,
        upload: UploadIcon,
        share: ShareIcon,
        eye: EyeIcon,
        view: EyeIcon,
        "eye-slash": EyeSlashIcon,
        hidden: EyeSlashIcon,
        copy: CopyIcon,
        scissors: ScissorsIcon,
        cut: ScissorsIcon,
        palette: PaletteIcon,
        color: PaletteIcon,
        sliders: SlidersIcon,
        options: SlidersIcon,
        "chart-line": ChartLineIcon,
        chart: ChartLineIcon,
        "chart-bar": ChartBarIcon,
        percent: PercentIcon,
        coins: CoinsIcon,
        wallet: WalletIcon,
        bank: BankIcon,
        "hand-coins": HandCoinsIcon,
        payment: HandCoinsIcon,
        "arrows-clockwise": ArrowsClockwiseIcon,
        sync: ArrowsClockwiseIcon,
        refresh: ArrowsClockwiseIcon,
        "arrow-right": ArrowRightIcon,
        "arrow-left": ArrowLeftIcon,
        "caret-down": CaretDownIcon,
        "caret-right": CaretRightIcon,
        "dots-three": DotsThreeIcon,
        more: DotsThreeIcon,
        // Product-related aliases
        product: PackageIcon,
        variant: StackIcon,
        category: FolderIcon,
        price: CurrencyDollarIcon,
        translation: GlobeIcon,
        version: ArrowsClockwiseIcon,
        rate: PercentIcon,
        exchange: ArrowsClockwiseIcon,
        generation: ArrowsClockwiseIcon,
        vat: PercentIcon,
        profile: UserIcon,
        item: ListIcon,
    };

    // Get icon - use map lookup, fallback to TableIcon
    function getIcon(iconName: string): Component<{ size?: number; class?: string }> {
        const normalized = iconName.toLowerCase().trim();
        return iconMap[normalized] ?? iconMap[fallback] ?? TableIcon;
    }

    let Icon = $derived(getIcon(name));
</script>

<Icon class={className} {size} />
