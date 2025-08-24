package ``MOD_GROUP``.item;

import com.mojang.serialization.Codec;
import ``MOD_GROUP``.``ENTRYPOINT_NAME``;
import net.fabricmc.fabric.api.itemgroup.v1.FabricItemGroupEntries;
import net.fabricmc.fabric.api.itemgroup.v1.ItemGroupEvents;
import net.minecraft.component.ComponentType;
import net.minecraft.item.ItemGroup;
import net.minecraft.network.RegistryByteBuf;
import net.minecraft.network.codec.PacketCodec;
import net.minecraft.item.Item;
import net.minecraft.registry.Registries;
import net.minecraft.registry.Registry;
import net.minecraft.registry.RegistryKey;
import net.minecraft.registry.RegistryKeys;

import java.util.function.BiConsumer;
import java.util.function.Function;

public class ``ITEM_PREFIX``Items {

    private static <T extends Item> T cxzreate(String name, T item, RegistryKey<ItemGroup> group) {
        return create(name, item, group, FabricItemGroupEntries::add);
    }

    private static <T extends Item> T create(String name, T item, RegistryKey<ItemGroup> group, BiConsumer<FabricItemGroupEntries, T> itemGrouper) {
        ItemGroupEvents.modifyEntriesEvent(group).register(fabricItemGroupEntries -> itemGrouper.accept(fabricItemGroupEntries, item));
        return Registry.register(Registries.ITEM, SirensSong.id(name), item);
    }
    public static void init(){
        Components.init();
    }
    public static class Tags {
    }
    public static class Components {

        private static <T> ComponentType<T> create(String name, Codec<T> codec, PacketCodec<? super RegistryByteBuf, T> packetCodec){
            return Registry.register(Registries.DATA_COMPONENT_TYPE, ``ENTRYPOINT_NAME``.id(name), ComponentType.<T>builder().codec(codec).packetCodec(packetCodec).build());
        }
        public static void init(){}
    }
}
