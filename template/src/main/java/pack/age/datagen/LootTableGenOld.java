package ``MOD_GROUP``.datagen;

import net.fabricmc.fabric.api.datagen.v1.FabricDataOutput;
import net.fabricmc.fabric.api.datagen.v1.provider.FabricBlockLootTableProvider;
import net.fabricmc.fabric.api.datagen.v1.provider.SimpleFabricLootTableProvider;
import net.minecraft.loot.LootTable;
import net.minecraft.loot.context.LootContextType;
import net.minecraft.loot.context.LootContextTypes;
import net.minecraft.registry.RegistryKey;
import net.minecraft.registry.RegistryWrapper;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
import java.util.function.BiConsumer;
public abstract class LootTableGen extends SimpleFabricLootTableProvider {
       protected final RegistryWrapper.WrapperLookup registryLookup;

    protected LootTableGen(FabricDataOutput dataOutput, CompletableFuture<RegistryWrapper.WrapperLookup> registryLookup, LootContextType contextType) {
        super(dataOutput, registryLookup, contextType);
        try {
            this.registryLookup = registryLookup.get();
        } catch (InterruptedException | ExecutionException e) {
            LOGGER.error("Seriously mojang made a better datagen system than you");
            throw new RuntimeException(e);
        }
    }

    public static class Block extends FabricBlockLootTableProvider {
        protected Block(FabricDataOutput dataOutput, CompletableFuture<RegistryWrapper.WrapperLookup> registryLookup) {
            super(dataOutput, registryLookup);
        }

        @Override
        public void generate() {

        }
    }

    public static class Entity extends LootTableGen {
        public Entity(FabricDataOutput output, CompletableFuture<RegistryWrapper.WrapperLookup> registryLookup) {
            super(output, registryLookup, LootContextTypes.ENCHANTED_ENTITY);
        }

        @Override
        public void accept(BiConsumer<RegistryKey<LootTable>, LootTable.Builder> lootTableBiConsumer) {

        }


    }
}
