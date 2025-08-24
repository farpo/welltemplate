package ``MOD_GROUP``.block;

import ``MOD_GROUP``.``ENTRYPOINT_NAME``;
import net.fabricmc.fabric.api.itemgroup.v1.FabricItemGroupEntries;
import net.fabricmc.fabric.api.itemgroup.v1.ItemGroupEvents;
import net.minecraft.block.Block;
import net.minecraft.item.BlockItem;
import net.minecraft.item.Item;
import net.minecraft.item.ItemGroup;
import net.minecraft.registry.Registries;
import net.minecraft.registry.Registry;
import net.minecraft.registry.RegistryKey;

import java.util.function.BiConsumer;
import java.util.function.Function;

public class ``BLOCK_PREFIX``Blocks {
    private static <T extends Block> T createWithItem(String name, T block, RegistryKey<ItemGroup> group){
        return createWithItem(name, block, group, FabricItemGroupEntries::add);
    }
    private static <T extends Block> T createWithItem(String name, T block, RegistryKey<ItemGroup> group, BiConsumer<FabricItemGroupEntries, Item> itemGrouper){
        return createWithItem(name, block, group, b -> new BlockItem(b, new Item.Settings()), itemGrouper);
    }
    private static <T extends Block> T createWithItem(String name, T block, RegistryKey<ItemGroup> group,
                                                      Function<T, Item> itemFactory,
                                                      BiConsumer<FabricItemGroupEntries, Item> itemGrouper){
        Item item = itemFactory.apply(block);
        create(name, block);
        ItemGroupEvents.modifyEntriesEvent(group).register(fabricItemGroupEntries -> itemGrouper.accept(fabricItemGroupEntries, item));
        Registry.register(Registries.ITEM, ``ENTRYPOINT_NAME``.id(name), item);
        return block;
    }
    private static <T extends Block> T create(String name, T block){
        return Registry.register(Registries.BLOCK, ``ENTRYPOINT_NAME``.id(name), block);
    }
    public static void init(){}
    public static class Tags{

    }
}
