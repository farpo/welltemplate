package ``MOD_GROUP``.block;

import ``MOD_GROUP``.``ENTRYPOINT_NAME``;
import net.fabricmc.fabric.api.itemgroup.v1.FabricItemGroupEntries;
import net.fabricmc.fabric.api.itemgroup.v1.ItemGroupEvents;
import net.minecraft.item.BlockItem;
import net.minecraft.item.Item;
import net.minecraft.item.ItemConvertible;
import net.minecraft.item.ItemGroup;
import net.minecraft.registry.Registries;
import net.minecraft.registry.Registry;
import net.minecraft.registry.RegistryKey;
import net.minecraft.registry.RegistryKeys;
import net.minecraft.block.AbstractBlock;
import net.minecraft.block.Block;

import java.util.function.BiConsumer;
import java.util.function.Function;

public class ``BLOCK_PREFIX``Blocks {

    private static <T extends Block> T createWithItem(String name, Function<AbstractBlock.Settings, T> constructor, AbstractBlock.Settings settings, RegistryKey<ItemGroup> itemGroup){
        return createWithItem(name, constructor, settings, itemGroup, FabricItemGroupEntries::add);
    }
    private static <T extends Block> T createWithItem(String name, Function<AbstractBlock.Settings, T> constructor, AbstractBlock.Settings settings, RegistryKey<ItemGroup> itemGroup, ItemConvertible after){
        return createWithItem(name, constructor, settings, itemGroup, (fabricItemGroupEntries, item) -> fabricItemGroupEntries.addAfter(after, item));
    }
    private static <T extends Block> T createWithItem(String name, Function<AbstractBlock.Settings, T> constructor, AbstractBlock.Settings settings, RegistryKey<ItemGroup> itemGroup, BiConsumer<FabricItemGroupEntries, Item> itemGrouper){
        RegistryKey<Item> key = RegistryKey.of(RegistryKeys.ITEM, ``ENTRYPOINT_NAME``.id(name));
        T block = create(name, constructor, settings);
        Item item = new BlockItem(block, new Item.Settings().registryKey(key));
        Registry.register(Registries.ITEM, key, item);
        ItemGroupEvents.modifyEntriesEvent(itemGroup).register(entries -> itemGrouper.accept(entries, item));
        return block;
    }
    private static <T extends Block> T create(String name, Function<AbstractBlock.Settings, T> constructor, AbstractBlock.Settings settings){
        RegistryKey<Block> key = RegistryKey.of(RegistryKeys.BLOCK, ``ENTRYPOINT_NAME``.id(name));
        settings.registryKey(key);
        return Registry.register(Registries.BLOCK, key, constructor.apply(settings));
    }
    public static void init(){}
    public static class Tags{

    }
}
